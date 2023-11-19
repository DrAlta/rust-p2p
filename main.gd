extends Control
@onready var offer_widget = $OfferPanel

@export var user_offer_scene: PackedScene

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
	offer_scene.connect("request_offer_copy", request_offer_copy)
	offer.connect("offer_generated", on_incoming_offer_generated)
	offer.create_offer()
	$Open/HBox/UserOffers.add_child(offer_scene)
	


func request_offer_copy(id):
	var jsoned = network.get_offer_by_id(id).get_json()
	if jsoned:
		DisplayServer.clipboard_set(jsoned)
	else:
		logy("error", "[main:46]failed to get JSON for offer " + str(id))


func request_offer_show(id):
	var jsoned = network.get_offer_by_id(id).get_json()
	if jsoned:
		showed_offer = id
		offer_widget.set_offer(jsoned)
	else:
		logy("error", "[main:55]failed to get JSON for offer " + str(id))



func _on_connection_input_request_offer():
	logy("trace", "[main:60]_on_connection_input_request_offer()")
	create_offer()


func on_incoming_offer_generated(dict_offer):
	logy("trace", "[main:65] on_offer_generated(offer)")
	if dict_offer.ID == showed_offer:
		var jsoned = network.get_offer_json_by_id(dict_offer.ID);
		if jsoned:
			offer_widget.set_offer(jsoned)


func _on_show_open_pressed():
	$Open.show()
	pass # Replace with function body.


func _on_open_close_pressed():
	logy("trace", "[main:78]_on_open_close_pressed()")
	$Open.hide()
	pass # Replace with function body.


func _on_connect_pressed():
	$ConnectionInput.show()
	pass # Replace with function body.


func logy(lvl, msg):
	print(lvl, msg)
