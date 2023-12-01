extends  Node
class_name RustNetwork
signal user_answer(id)
signal user_answer_completed(id)
signal user_offer_completed(id)
signal offer_generated(ided_offer: Dictionary)
var links := {}
var established := {}

var rust_logic : RustLogic


func _init():
	rust_logic = RustLogic.new()


func _process(_delta):
	#logy("trace", "[network:43]_process()")
	for link in links:
		process_link(links[link])
	var processing_commands : = true
	while processing_commands:
		var x = rust_logic.poll();
		if x.Command != "Null":
			print("    ", x.Command)
		match x:
			{"Command": "Null"}:
				processing_commands = false
			{"Command": "AddICE", "ChannelID": var channel_id, "Index": var index, "Media": var media, "Name": var name_arg}:
				links[channel_id].add_ice_candidate(media, index, name_arg)
			{"Command": "AnswerOffer", "ChannelID": var channel_id, "Answer": var answer}:
				logy("debug", "[rustnetwork:32] got AnswerOffer for channel " + str(channel_id))
				links[channel_id].give_answer(answer)
			{"Command": "GenerateAnswer", "ChannelID": var channel_id, "Offer": var offer}:
				create_outgoing(channel_id, offer)
			{"Command": "GenerateOffer", "ChannelID": var channel_id}:
				links[channel_id].create_offer()
			{"Command": "Send", "ChannelID": var channel_id, "Packet": var packet}:
				send(channel_id, packet)
			{"Command": "SendDirect", "ChannelID": var channel_id, "Packet": var packet}:
				send(channel_id, packet)
			{"Command": "UserAnswer", "ChannelID": var channel_id, "Answer": var _answer}:
				emit_signal("user_answer", channel_id)
			{"Command": "UserOffer", "ChannelID": var channel_id, "Offer": var _offer}:
				print("[rustnetwork:41] emiting UserOffer signal")
				emit_signal("offer_generated", channel_id)
			_:
				print("unhandled packet", JSON.stringify(x))

func create_incoming():
	var id = rust_logic.generate_offer()
	var link := P2PLink.new(id)
	links[id] = link
	link.connect("offer_generated", on_offer_generated)
	link.connect("new_ice_candidate", on_new_ice_candidate)

	return id


func create_outgoing(id, offer):
	var link := P2PLink.new(id)
	links[id] = link
	link.connect("answer_generated", on_answer_generated)
	link.connect("new_ice_candidate", on_new_ice_candidate)
	link.set_remote_description("offer", offer)

	return id


func get_answer_json_by_id(id):
	var json = rust_logic.get_answer_json_by_id(id)
	if json == "":
		return null
	else:
		return json


func get_offer_json_by_id(id):
	var json = rust_logic.get_offer_json_by_id(id)
	if json == "":
		return null
	else:
		return json

func process_link(link:P2PLink):
	var id = link.id
	link.poll()
	if link.get_ready_state() == WebRTCDataChannel.STATE_OPEN:
		logy("debug", "[rustnetwork:89] link:" + str(id) + " ready")
		if not established.has(id):
			logy("trace", "[rustnetwork:89] connection " + str(id) + " established")
			established[id] = true
			rust_logic.channel_established(id)
		if link.get_available_packet_count() > 0:
			rust_logic.receive(id, link.get_packet())


func send(channel_id, packet: String):
	if channel_id == 0:
		print("Packet for user:", packet)
	else:
		links[channel_id].send(packet)


func user_packet(packet: String):
	print("UP:", packet)
	rust_logic.receive(0, packet)


func on_answer_generated(dict_answer):
	logy("signal", "[rustnetwork:109]on_answer_generated(dict_answer)")
	rust_logic.on_answer_generated(dict_answer.ID, dict_answer.Answer)


func on_new_ice_candidate(id, mid_name, index_name, sdp_name):
	rust_logic.on_new_ice_candidate(id, mid_name, index_name, sdp_name)


func on_offer_generated(dict_offer):
	print("[rustnetwork:90]on_offer_generated(dict_offer)")
	rust_logic.on_offer_generated(dict_offer.ID, dict_offer.Offer)


func logy(lvl: String, msg: String):
	print(lvl, msg)

