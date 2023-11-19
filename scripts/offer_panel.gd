extends PanelContainer


# Called when the node enters the scene tree for the first time.
func _ready():
	
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass

func set_offer(msg:String):
	$HBoxContainer/VBoxContainer/OfferHBoxContainer/OfferText.text = msg
	show()

func copy_offer_to_clipboard():
	DisplayServer.clipboard_set($HBoxContainer/VBoxContainer/OfferHBoxContainer/OfferText.text)


func _on_close_pressed():
	logy("trace", "[offer_panel:23]_on_close_pressed()")
	hide()
	pass # Replace with function body.


func logy(lvl, msg):
	print(lvl, msg)
