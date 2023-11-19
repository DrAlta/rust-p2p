class_name P2PNetwork

var my_version = "0.0"

var my_id : String

var open_incoming : = {}

var my_peers : = {}



func create_incoming() -> String:
	var id : = "Incoming" + str(open_incoming.size())
	var incoming= P2PIncoming.new(id)
	open_incoming[id] = incoming
	return id

func get_incoming_by_id(id: String) -> P2PIncoming:
	var incoming = open_incoming[id]
	if incoming:
		return incoming
	else:
		print("tried to get P2PIncoming ", id, " which doesn't exist")
		return null


func get_offer_json_by_id(id: String):
	var incoming = open_incoming[id]
	if incoming.offer != "":
		return JSON.stringify({"Source": my_id, "ID" : incoming.link.id, "Offer" : incoming.offer, "ICE" : incoming.link.ice})
	else:
		logy("error", "[network:33]asked for JSONfied of offer"+  str(id) + " but doesn't have a webRTC offer yet")
		return null


func _init():
	my_id = str(randi())


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(_delta):
	#logy("trace", "[network:43]_process()")
	for key in open_incoming:
		var incoming = open_incoming[key]
		process_incoming(incoming)


func logy(lvl, msg):
	print(lvl, msg)


func process_incoming(incoming:P2PIncoming):
	incoming.poll()
	if incoming.get_ready_state() == WebRTCDataChannel.STATE_OPEN:
		if not incoming.greeted_ka:
			incoming.send(JSON.stringify({"Type":"Greetings", "Me": my_id, "Version": my_version}))
		if incoming.get_available_packet_count() > 0:
			var raw_packet = incoming.get_packet()
			var packet = JSON.parse_string(raw_packet)
			if packet:
				match packet:
					{"Type" : "Greeting", "Me" : var who, "Version": var version}:
						if version != my_version:
							incoming.send(JSON.stringify({"Type":"UnknownVersion"}))
						else:
							var new_id = str(who)
							if new_id in my_peers:
								logy("bootstrap", "incoming " + str(incoming.link.id) + " introduced itself as " + str(who) + " again...")
								incoming.send(JSON.stringify({"Type":"Not you again!"}))
								remove_incoming(incoming.link.id)
							else:
								logy("bootstrap", "incoming " + str(incoming.link.id) + " introduced itself as " + str(who))
								open_incoming.erase(incoming.link.id)
								incoming.link.id = new_id
								my_peers[str(new_id)] = incoming.link
								incoming.free()
					{"Type" : "Me", "Me" : var who}:
						var new_id = str(who)
						if new_id in my_peers:
							logy("bootstrap", "incoming " + str(incoming.link.id) + " identified itself as " + str(who) + " again...")
							incoming.send(JSON.stringify({"Type":"Not you again!"}))
							remove_incoming(incoming.link.id)
						else:
							logy("bootstrap", "incoming " + str(incoming.link.id) + " identified itself as " + str(who))
							open_incoming.erase(incoming.link.id)
							incoming.link.id = new_id
							my_peers[str(new_id)] = incoming.link
							incoming.free()
					{"Type" : "Who"}:
						incoming.send(JSON.stringify({"Type":"Me", "Me": my_id}))
					_ :
						logy("bootstrap_error", str(incoming.link.id) + " sent unhandleed message:" + raw_packet)
						remove_incoming(incoming.link.id)
			else:
				logy("bootstrap_error", str(incoming.link.id) + " sent invaled message:" + raw_packet)
					
func remove_incoming(id:String):
	var incoming = open_incoming[id]
	if incoming:
		incoming.close()
		open_incoming.erase(id)
		incoming.free()

			
