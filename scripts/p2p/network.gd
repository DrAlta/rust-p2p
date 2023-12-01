extends  Node
class_name P2PNetwork
signal user_answer(id)
signal user_answer_completed(id)
signal user_offer_completed(id)
signal offer_generated(id)
var my_version = "Rust:0.0"

var my_id : String

var open_incoming : = {}
var open_outgoing : = {}

#{id: link}
var my_neighbors : = {}



func _init():
	my_id = str(randi())
	print("[network:17] generatin id", my_id)


func _process(_delta):
	#logy("trace", "[network:43]_process()")
	for key in open_incoming:
		var incoming = open_incoming[key]
		process_incoming(incoming)
	for key in open_outgoing:
		var outgoing = open_outgoing[key]
		process_outgoing(outgoing)


func create_incoming() -> String:
	var id : = "Incoming" + str(open_incoming.size())
	var incoming := P2PIncoming.new(id)
	incoming.connect("offer_generated", on_incoming_offer_generated)
	incoming.create_offer()

	open_incoming[id] = incoming
	add_child(incoming)
	return id


func create_outgoing(offer_id: String, destination: String) -> String:
	var id : = "Outgoing" + str(open_outgoing.size())
	var outgoing= P2POutgoing.new(id, offer_id, destination)
	open_outgoing[id] = outgoing
	add_child(outgoing)
	return id


func get_answer_json_by_id(id: String):
	if open_outgoing.has(id):
		var outgoing = open_outgoing[id]
		if outgoing.answer != "":
			return JSON.stringify({"Source": my_id, "Destination": outgoing.destination, "Type": {"Answer": {"Answer": outgoing.answer, "OfferID": outgoing.offer_id, "ICE" : outgoing.link.ice}}})
		else:
			logy("error", "[network:33]asked for JSONfied of offer"+  str(id) + " but doesn't have a webRTC offer yet")
		return null
	else:
		logy("error", "[network:68]asked for JSONfied of offer"+  str(id) + " but it doesn't exist")
	return null


func get_incoming_by_id(id: String) -> P2PIncoming:
	if open_incoming.has(id):
		return open_incoming[id]
	else:
		print("tried to get P2PIncoming ", id, " which doesn't exist")
		return null


func get_outgoing_by_id(id: String) -> P2POutgoing:
	if open_outgoing.has(id):
		return open_outgoing[id]
	else:
		print("tried to get P2POutgoing ", id, " which doesn't exist")
		return null


func get_offer_json_by_id(id: String):
	if open_incoming.has(id):
		var incoming = open_incoming[id]
		if incoming.offer != "":
			return JSON.stringify({"Destination": "", "Source": my_id,  "Type": {"Offer": { "Offer" : incoming.offer, "OfferID" : incoming.link.id, "ICE" : incoming.link.ice}}})
		else:
			logy("error", "[network:33]asked for JSONfied of offer"+  str(id) + " but doesn't have a webRTC offer yet")
		return null
	else:
		logy("error", "[network:68]asked for JSONfied of offer"+  str(id) + " but it doesn't exist")
	return null


func process_incoming(incoming:P2PIncoming):
	incoming.poll()
	if incoming.get_ready_state() == WebRTCDataChannel.STATE_OPEN:
		if not incoming.greeted_ka:
			incoming.greeted_ka = true
			incoming.send(JSON.stringify({"Type": "Greetings", "Me": my_id, "Version": my_version}))
		if incoming.get_available_packet_count() > 0:
			var raw_packet = incoming.get_packet()
			var packet = JSON.parse_string(raw_packet)
			if packet:
				match packet:
					{"Type": "Greetings", "Me": var who, "Version": var version}:
						if version != my_version:
							incoming.send(JSON.stringify({"Type":"UnknownVersion"}))
						else:
							var new_id = str(who)
							if new_id in my_neighbors:
								logy("bootstrap", "incoming " + str(incoming.link.id) + " introduced itself as " + str(who) + " again...")
								incoming.send(JSON.stringify({"Type":"NotYouAgain"}))
								remove_incoming(incoming.link.id)
							else:
								logy("bootstrap", "incoming " + str(incoming.link.id) + " introduced itself as " + str(who))
								open_incoming.erase(incoming.link.id)
								incoming.link.id = new_id
								my_neighbors[str(new_id)] = incoming.link
								incoming.free()
					{"Type": "Me", "Me": var who}:
						var new_id = str(who)
						if new_id in my_neighbors:
							logy("bootstrap", "incoming " + str(incoming.link.id) + " identified itself as " + str(who) + " again...")
							incoming.send(JSON.stringify({"Type":"Not you again!"}))
							remove_incoming(incoming.link.id)
						else:
							logy("bootstrap", "incoming " + str(incoming.link.id) + " identified itself as " + str(who))
							open_incoming.erase(incoming.link.id)
							incoming.link.id = new_id
							my_neighbors[str(new_id)] = incoming.link
							incoming.free()
					{"Type" : "Who"}:
						incoming.send(JSON.stringify({"Type": "Me", "Me": my_id}))
					_ :
						logy("bootstrap_error", str(incoming.link.id) + " sent unhandleed message:" + raw_packet)
						remove_incoming(incoming.link.id)
			else:
				logy("bootstrap_error", str(incoming.link.id) + " sent invaled message:" + raw_packet)
				incoming.send(JSON.stringify({"Type":"InvalidPacket"}))
				remove_incoming(incoming.link.id)


func process_outgoing(outgoing:P2POutgoing):
	outgoing.poll()
	if outgoing.get_ready_state() == WebRTCDataChannel.STATE_OPEN:
		if not outgoing.greeted_ka:
			outgoing.greeted_ka = true
			outgoing.send(JSON.stringify({"Type": "Greetings", "Me": my_id, "Version": my_version}))
		if outgoing.get_available_packet_count() > 0:
			var raw_packet = outgoing.get_packet()
			var packet = JSON.parse_string(raw_packet)
			if packet:
				match packet:
					{"Type": "Greetings", "Me": var who, "Version": var version}:
						if version != my_version:
							outgoing.send(JSON.stringify({"Type":"UnknownVersion"}))
						else:
							var new_id = str(who)
							if new_id != outgoing.link.id:
								logy("bootstrap", "[network:155]unknown outgoing " + str(outgoing.link.id) + " introduced itself as " + str(who))
							else:
								logy("bootstrap", "[network:157]outgoing " + str(outgoing.link.id) + " introduced itself as " + str(who))
							if my_neighbors.has(new_id):
								say_goodbye_to(new_id)
							open_outgoing.erase(outgoing.link.id)
							outgoing.link.id = new_id
							my_neighbors[new_id] = outgoing.link
							outgoing.queue_free()
					{"Type" : "Who"}:
						outgoing.send(JSON.stringify({"Type":"Me", "Me": my_id}))
					_ :
						logy("bootstrap_error", str(outgoing.link.id) + " sent unhandleed message:" + raw_packet)
						outgoing.send(JSON.stringify({"Type":"InvalidSalutation"}))
						remove_outgoing(outgoing.link.id)
			else:
				logy("bootstrap_error", str(outgoing.link.id) + " sent invaled message:" + raw_packet)
				outgoing.send(JSON.stringify({"Type":"InvalidPacket"}))
				remove_outgoing(outgoing.link.id)

func process_packet(packet:Dictionary):
	print("MyID:", my_id)
	match packet:
		{"Destination": my_id, "Source": var source, "Type" : "Who"}:
			logy("packet", "[network:174] processing Who packet")
			send({"Source": my_id, "Destination": source, "Type": {"Me": {"Me": my_id}}})
		{"Destination": my_id,  "Source": _, "Type": {"Answer": {"Answer": var answer, "OfferID": var offer_id, "ICE": var ice}}}:
			logy("packet", "[network:177] processing Answer packet")
			if open_incoming.has(offer_id):
				var incoming = open_incoming[offer_id]
				incoming.give_answer(answer)
				for thing in ice:
					incoming.add_ice_candidate(thing.Media, thing.Index, thing.Name)
				emit_signal("user_offer_completed", offer_id)
		{"Destination": my_id, "Source": var source, "Type": {"Offer": { "OfferID" : var offer_id, "Offer" : var offer, "ICE" : var ice,}}}:
			logy("packet", "[newtork:185] processing Offer packet")
			var id = create_outgoing(offer_id, source)
			var outgoing: P2POutgoing = get_outgoing_by_id(id)
			outgoing.connect("answer_generated", on_outgoing_answer_generated)
			outgoing.connect("new_ice_candidate", on_outgoing_new_ice_candidate)
			outgoing.set_remote_description("offer", offer)
			for thing in ice:
				outgoing.add_ice_candidate(thing.Media, thing.Index, thing.Name)
		{"Destination": my_id, .. }:
			logy("error", "[network:199]" + str(packet.Source) + " sent me unhandleed message:" + str(packet.keys()))
		{"Destination": "", "Source": var source, "Type":{"Offer": { "OfferID" : var offer_id, "Offer" : var offer, "ICE" : var ice,}}}:
			logy("trace", "[network:201]" + str(packet.Source) + " user offer")
			var id = create_outgoing(offer_id, source)
			var outgoing: P2POutgoing = get_outgoing_by_id(id)
			outgoing.connect("answer_generated", on_outgoing_user_answer_generated)
			outgoing.connect("new_ice_candidate", on_outgoing_new_ice_candidate)
			outgoing.set_remote_description("offer", offer)
			for thing in ice:
				outgoing.add_ice_candidate(thing.Media, thing.Index, thing.Name)
		{"Destination": "", .. }:
			logy("trace", "[network:203]" + str(packet.Source) + " user packet:" + JSON.stringify(packet.Type))
		{"Destination": _, .. }:
			send(packet)
		_:
			logy("error", "[newtork:196]" + str(packet.Source) + " sent unhandleed message:" + str(packet.keys()))


func remove_incoming(id:String):
	if open_incoming.has(id):
		var incoming = open_incoming[id]
		incoming.close()
		open_incoming.erase(id)
		incoming.queue_free()


func remove_outgoing(id:String):
	if open_outgoing.has(id):
		var outgoing = open_outgoing[id]
		outgoing.close()
		open_outgoing.erase(id)
		outgoing.queue_free()
		emit_signal("user_answer_completed", id)


func say_goodbye_to(id: String):
	if my_neighbors.has(id):
		var link: P2PLink = my_neighbors[id]
		link.send(JSON.stringify({"Type":"Goodbye"}))
		link.close()
		my_neighbors.erase(id)


func send(packet: Dictionary):
	logy("trace", "[network:116]send()")
	if packet.has("Destination"):
		var dest = str(packet.Destination)
		if my_neighbors.has(dest):
			var link: P2PLink = my_neighbors[dest]
			var jsoned := JSON.stringify(packet)
			link.send(jsoned)
		else:
			logy("error", "[network:124]No known route to" + str(dest))
	else:
		logy("error", "[network:126]Packet has no Destination")
	


func user_packet(msg):
	var packet = JSON.parse_string(msg)
	if packet:
		logy("trace", "[network:250] user submitted:" + str(packet.keys()) + " to " + str(packet.Destination))
		if not packet.has("Source"):
			packet.Source = "User"
		process_packet(packet)
	else:
		logy("error", "Could parse user packet as json")


func on_incoming_offer_generated(dict_offer):
	logy("signal", "[network:275] on_offer_generated(dict_offer)")
	emit_signal("offer_generated", dict_offer.ID)

func on_outgoing_answer_generated(dict_answer: Dictionary):
	logy("signal", "[network:268]on_outgoing_answer_generated(dict_answer: Dictionary)")
	if open_outgoing.has(dict_answer.ID):
		var outgoing = open_outgoing[dict_answer.ID]
		logy("debug", "[network:271] it was answer for " + str(outgoing.destination))
		send({"Source": my_id, "Destination": outgoing.destination, "Type": {"Answer": {"Answer": dict_answer.Answer, "OfferID": dict_answer.OfferID, "ICE": dict_answer.ICE}}})
	else:
		logy("error", "[network:274] got an `answer_generated` for" + str(dict_answer.ID) + "which I couldn't find")


func on_outgoing_new_ice_candidate(id, offer_id, mid_name, index_name, sdp_name):
	logy("signal", "[network:289]on_outgoing_new_ice_candidate(id, offer_id, mid_name, index_name, sdp_name)")
	if open_outgoing.has(id):
		var outgoing = open_outgoing[id]
		if outgoing.destination == "":
			emit_signal("user_answer", id)
		else:
			send({
				"Source": my_id, 
				"Destination": outgoing.destination, 
				"Type": "newICE", 
				"OfferID": offer_id, 
				"ICE": [
					{
						"Media" : mid_name, 
						"Index" : index_name, 
						"Name" : sdp_name
					}
				]
			})
	else:
		logy("error", "[network:309] Couldn't find outgoing with id" + str(id))


func on_outgoing_user_answer_generated(dict_answer: Dictionary):
	logy("signal", "[network:302]on_outgoing_user_answer_generated(dict_answer: Dictionary)")
	if open_outgoing.has(dict_answer.ID):
		var outgoing = open_outgoing[dict_answer.ID]
		logy("debug", "[network:305] it was a user answer")
		emit_signal("user_answer", dict_answer.ID)
	else:
		logy("error", "[network:308] got an `answer_generated` for" + str(dict_answer.ID) + "which I couldn't find")

func logy(lvl: String, msg: String):
	print(lvl, msg)

