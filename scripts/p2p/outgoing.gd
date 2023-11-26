extends Node
class_name P2POutgoing
signal new_ice_candidate(id, offer_id, mid_name, index_name, sdp_name)
signal answer_generated(dict_answer: Dictionary)

var link: P2PLink
var answer : = ""
var offer_id: String
var destination := ""
var greeted_ka : = false

func _init(id_arg: String, offer_id_arg: String):
	offer_id = offer_id_arg
	link = P2PLink.new(id_arg)
	link.connect("answer_generated", on_answer_generated)
	link.connect("new_ice_candidate", on_new_ice_candidate)


func add_ice_candidate(media: String, index: int, name:String):
	link.add_ice_candidate(media, index, name)


func close():
	link.close()


func get_available_packet_count() -> int:
	return link.get_available_packet_count()


func get_packet() -> String:
	return link.get_packet()


func get_ready_state() -> WebRTCDataChannel.ChannelState:
	return link.channel.get_ready_state()


func poll() -> Error:
	return link.poll()


func send(msg:String):
	link.send(msg)


func set_remote_description(type: String, sdp: String):
	link.set_remote_description(type, sdp)


func on_answer_generated(dict_answer):
	logy("signal", "[outgoming:51]on_answer_generated(dict_answer)")
	await get_tree().create_timer(1).timeout
	answer = dict_answer.Answer
	emit_signal("answer_generated", {"ID" : dict_answer.ID, "OfferID": offer_id, "Answer" : dict_answer.Answer, "ICE" : link.ice})


func on_new_ice_candidate(id, mid_name, index_name, sdp_name):
	logy("signal", "[outgoing:57]on_new_ice_candidate(id, mid_name, index_name, sdp_name)")
	emit_signal("new_ice_candidate", id, offer_id, mid_name, index_name, sdp_name)


func logy(lvl: String, msg: String):
	print(lvl, msg)
