extends  Node
class_name P2PIncoming
signal new_ice_candidate(id, mid_name, index_name, sdp_name)
signal offer_generated(ided_offer: Dictionary)
var link: P2PLink
var offer : = ""
var greeted_ka : = false


func _init(id_arg: String):
	link = P2PLink.new(id_arg)
	link.connect("offer_generated", on_offer_generated)
	link.connect("new_ice_candidate", on_new_ice_candidate)


func add_ice_candidate(media: String, index: int, name_arg:String):
	link.add_ice_candidate(media, index, name_arg)


func close():
	link.close()


func create_offer():
	logy("trace", "[incoming:21]create_offer()")
	link.create_offer()


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


func give_answer(answer: String):
	link.give_answer(answer)

func on_new_ice_candidate(id, mid_name, index_name, sdp_name):
	logy("signal", "[incoming:53]on_new_ice_candidate(id, mid_name, index_name, sdp_name)")
	emit_signal("new_ice_candidate", id, mid_name, index_name, sdp_name)
	if offer != "":
		emit_signal("offer_generated", {"ID" : id, "Offer" : offer, "ICE" : link.ice})

func on_offer_generated(dict_offer):
	logy("signal", "[incoming:59]on_offer_generated(ided_offer)")
	offer = dict_offer.Offer
	await get_tree().create_timer(1).timeout
	emit_signal("offer_generated", {"ID" : dict_offer.ID, "Offer" : dict_offer.Offer, "ICE" : link.ice})

func logy(lvl: String, msg: String):
	print(lvl, msg)
