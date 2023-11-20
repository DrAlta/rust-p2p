extends Control
@export var user_offer_scene: PackedScene

@onready var gui_answer_panel : = $AnswerPanel
@onready var gui_offer_panel : = $OfferPanel
var network : P2PNetwork
var showed_offer: String

class UserOffer:
	var node: Node
	var data: String
	
	func _init(data_arg: String, node_arg: Node):
		node = node_arg
		data = data_arg

# Called when the node enters the scene tree for the first time.
func _ready():
	network = P2PNetwork.new()


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
	logy("error", "[main:43]on_request_offer_copy(id)")
	var jsoned = network.get_offer_by_id(id).get_json()
	if jsoned:
		DisplayServer.clipboard_set(jsoned)
	else:
		logy("error", "[main:48]failed to get JSON for offer " + str(id))


func on_request_offer_show(id):
	logy("error", "[main:52]on_request_offer_show(id)")
	var jsoned = network.get_offer_by_id(id).get_json()
	if jsoned:
		showed_offer = id
		gui_offer_panel.set_offer(jsoned)
	else:
		logy("error", "[main:58]failed to get JSON for offer " + str(id))



func _on_connection_input_request_offer():
	logy("trace", "[main:63]_on_connection_input_request_offer()")
	create_offer()


func on_incoming_offer_generated(dict_offer):
	logy("trace", "[main:68] on_offer_generated(offer)")
	if dict_offer.ID == showed_offer:
		var jsoned = network.get_offer_json_by_id(dict_offer.ID);
		if jsoned:
			gui_offer_panel.set_offer(jsoned)


func _on_show_open_pressed():
	logy("trace", "[main:76]_on_show_open_pressed()")
	$Open.show()
	pass # Replace with function body.


func _on_open_close_pressed():
	logy("trace", "[main:82]_on_open_close_pressed()")
	$Open.hide()
	pass # Replace with function body.


func _on_connect_pressed():
	logy("trace", "[main:88]_on_connect_pressed()")
	$ConnectionInput.show()
	pass # Replace with function body.


func _on_connection(msg):
	logy("trace", "[main:94]_on_connection()")
	network.user_packet(msg)


func logy(lvl: String, msg: String):
	Logy.logy(lvl, msg)
