extends PanelContainer
var shown_id
func set_answer(id, msg: String):
	shown_id = id
	$HBoxContainer/OutgoingText.text = msg
	show()


func _on_close_pressed():
	logy("trace", "[answer_panel:8]_on_close_pressed()")
	hide()


func _on_copy_pressed():
	DisplayServer.clipboard_set($HBoxContainer/OutgoingText.text)


func logy(lvl: String, msg: String):
	print(lvl, msg)
