extends PanelContainer
signal request_offer_copy(id)
signal request_offer_show(id)

var id


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


func get_id():
	return id


func set_id(id_arg):
	id = id_arg
	$HBoxContainer/Label.text = str(id_arg)

func on_copy_button_pressed():
	logy("trace", "[user_offer:19]on_copy_button_pressed()")
	emit_signal("request_offer_copy", id)
	
func on_show_button_pressed():
	logy("trace", "[user_offer:23]on_show_button_pressed()")
	emit_signal("request_offer_show", id)


func logy(lvl: String, msg: String):
	print(lvl, msg)
