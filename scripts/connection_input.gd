extends PanelContainer
signal connection(data:String)
signal request_offer
# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_paste_pressed():
	$HBox/InputText.text = DisplayServer.clipboard_get()
	emit_signal("connection", $HBox/InputText.text)


func _on_enter_pressed():
	logy("trace", "[connection_input:20]_on_enter_pressed()")
	if $HBox/InputText.text == "":
		emit_signal("request_offer")
		hide()
	emit_signal("connection", $HBox/InputText.text)


func _on_close_pressed():
	hide()

func _on_input_text_text_changed():
	if $HBox/InputText.text == "":
		$HBox/VBoxContainer/Enter.text = "Generate\nOffer"
	else:
		$HBox/VBoxContainer/Enter.text = "Enter"


func logy(lvl, msg):
	print(lvl, msg)
