extends Control
@export var user_offer_scene: PackedScene

@onready var gui_answer_panel : = $AnswerPanel
@onready var gui_offer_panel : = $OfferPanel

@onready var gui_user_answers : = $Open/HBox/UserAnswers
@onready var gui_user_offers : = $Open/HBox/UserOffers
@onready var network : = $P2PNetwork
var showed_offer: String

class UserOffer:
	var node: Node
	var data: String
	
	func _init(data_arg: String, node_arg: Node):
		node = node_arg
		data = data_arg

# Called when the node enters the scene tree for the first time.
func _ready():
	var ctx = HashingContext.new()
	ctx.start(HashingContext.HASH_MD5)
	ctx.update("test string".to_utf8_buffer())
	print("[", ctx.finish().hex_encode(), "]")
#	network = P2PNetwork.new()


func _process(delta):
	#logy("trace", "[main:23]_process()")
	network._process(delta)


func create_offer():
	logy("trace", "[main:28]create_offer()")
	var id = network.create_incoming()
	showed_offer = id
	var offer =  network.get_incoming_by_id(id)
	var offer_scene = user_offer_scene.instantiate()
	offer_scene.set_id(id)
	offer_scene.connect("request_offer_show", on_request_offer_show)
	offer_scene.connect("request_offer_copy", on_request_offer_copy)
	offer.connect("offer_generated", on_incoming_offer_generated)
	offer.create_offer()
	$Open/HBox/UserOffers.add_child(offer_scene)
	


func on_request_offer_copy(id):
	logy("signal", "[main:43]on_request_offer_copy(id)")
	var jsoned = network.get_offer_by_id(id).get_json()
	if jsoned:
		DisplayServer.clipboard_set(jsoned)
	else:
		logy("error", "[main:48]failed to get JSON for offer " + str(id))


func on_request_offer_show(id):
	logy("signal", "[main:52]on_request_offer_show(id)")
	var jsoned = network.get_offer_by_id(id).get_json()
	if jsoned:
		showed_offer = id
		gui_offer_panel.set_offer(id, jsoned)
	else:
		logy("error", "[main:62]failed to get JSON for offer " + str(id))



func _on_connection_input_request_offer():
	logy("signal", "[main:67]_on_connection_input_request_offer()")
	create_offer()


func on_incoming_offer_generated(dict_offer):
	logy("signal", "[main:72] on_offer_generated(offer)")
	if dict_offer.ID == showed_offer:
		var jsoned = network.get_offer_json_by_id(dict_offer.ID);
		if jsoned:
			gui_offer_panel.set_offer(dict_offer.ID, jsoned)


func _on_show_open_pressed():
	logy("signal", "[main:80]_on_show_open_pressed()")
	$Open.show()
	pass # Replace with function body.


func _on_open_close_pressed():
	logy("signal", "[main:86]_on_open_close_pressed()")
	$Open.hide()
	pass # Replace with function body.


func _on_connect_pressed():
	logy("signal", "[main:88]_on_connect_pressed()")
	$ConnectionInput.show()
	pass # Replace with function body.


func _on_connection(msg):
	logy("signal", "[main:94]_on_connection()")
	network.user_packet(msg)


func _on_network_user_answer(id):
	logy("signal", "[main:106]_on_newtork_user_answer(id)")
	var jsoned = network.get_answer_json_by_id(id)
	gui_answer_panel.set_answer(id, jsoned)


func _on_network_user_answer_completed(id):
	logy("signal", "[main:112]_on_newtork_user_answer_completed(id)")
	if gui_answer_panel.shown_id == id:
		gui_answer_panel.hide()
	for child in gui_user_answers.get_children():
		if child.has_method("get_id"):
			if child.get_id() == id:
				child.queue_free()


func _on_network_user_offer_completed(id):
	logy("signal", "[main:120]_on_network_user_offer_completed(id)")
	if gui_offer_panel.shown_id == id:
		gui_offer_panel.hide()
	for child in gui_user_offers.get_children():
		if child.has_method("get_id"):
			if child.get_id() == id:
				child.queue_free()

func logy(lvl: String, msg: String):
	Logy.logy(lvl, msg)




