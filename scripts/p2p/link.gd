class_name  P2PLink

signal new_ice_candidate(id, mid_name, index_name, sdp_name)
signal offer_generated(dict_offer)
signal answer_generated(dict_answer)


var connection : WebRTCPeerConnection
var channel : WebRTCDataChannel
var id 
var ice = []

func _init(id_arg):
	logy("trace", "[link:14]_init()")
	id = id_arg
	connection = WebRTCPeerConnection.new()
	channel = connection.create_data_channel("main", {"negotiated": true, "id": 1})
	connection.session_description_created.connect(on_session_description_created)
	connection.ice_candidate_created.connect(on_new_ice_candidate)


func close():
	connection.close()
#	channel.free()
#	connection.free()


func create_offer():
	logy("trace", "[link:29]create_offer()")
	connection.create_offer()


func get_available_packet_count() -> int:
	return channel.get_available_packet_count()


func get_ready_state():
	return channel.get_ready_state()


func get_packet() -> String:
	return channel.get_packet().get_string_from_utf8()


func give_answer(answer: String):
	logy("debug", "[link:46]" + str(id) + " give_answer(answer)")
	connection.set_remote_description("answer", answer)


func poll() -> Error:
	#logy("trace", "[link:29]poll()")
	return connection.poll()

func send(msg:String) -> bool:
	if channel.get_ready_state() == WebRTCDataChannel.STATE_OPEN:
		channel.put_packet(msg.to_utf8_buffer())
		return true
	else:
		return false


func add_ice_candidate(a,b,c):
	logy("debug", "[link:63] adding ice to link:" + str(id))
	connection.add_ice_candidate(a,b,c)


func set_remote_description(type: String, sdp: String):
	connection.set_remote_description(type, sdp)


func on_new_ice_candidate(mid_name, index_name, sdp_name):
	logy("signal", "[link:70]on_new_ice_candidate()")
	ice.append({"Media" : mid_name, "Index" : index_name, "Name" : sdp_name})
	emit_signal("new_ice_candidate", id, mid_name, index_name, sdp_name)


func on_session_description_created(type, data):
	logy("signal", "[link:76]on_session_description_created()")
	connection.set_local_description(type, data)
	if type == "offer": 
		emit_signal("offer_generated", {"ID" : id, "Offer" : data})
	elif type == "answer":
		emit_signal("answer_generated", {"ID" : id, "Answer" : data})
	else:
		logy("impossible", "[link:83]This shouldn't happen")


func logy(lvl: String, msg: String):
	print(lvl, msg)

