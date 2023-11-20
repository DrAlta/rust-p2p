extends PanelContainer
signal connection(data:String)
signal request_offer
@onready var gui_input_text : = $HBox/InputText


func _on_paste_pressed():
	gui_input_text.text = DisplayServer.clipboard_get()
	if gui_input_text.text != "":
		emit_signal("connection", gui_input_text.text)


func _on_enter_pressed():
	logy("trace", "[connection_input:20]_on_enter_pressed()")
	if gui_input_text.text == "":
		emit_signal("request_offer")
		hide()
	else:
		emit_signal("connection", gui_input_text.text)


func _on_close_pressed():
	hide()


func _on_input_text_text_changed():
	if $HBox/InputText.text == "":
		$HBox/VBoxContainer/Enter.text = "Generate\nOffer"
	else:
		$HBox/VBoxContainer/Enter.text = "Enter"


func logy(lvl: String, msg: String):
	Logy.logy(lvl, msg)
