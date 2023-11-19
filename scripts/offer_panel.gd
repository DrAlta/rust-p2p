extends PanelContainer
signal  connetion(msg: String)

@onready var gui_answer_text = $HBoxContainer/VBoxContainer/AnswerHBox/AnswerText
@onready var gui_offer_text = $HBoxContainer/VBoxContainer/OfferHBoxContainer/OfferText

func set_offer(msg:String):
	gui_offer_text.text = msg
	show()

func copy_offer_to_clipboard():
	DisplayServer.clipboard_set(gui_offer_text.text)


func _on_close_pressed():
	logy("trace", "[offer_panel:25]_on_close_pressed()")
	hide()
	pass # Replace with function body.


func _on_enter_pressed():
	if gui_answer_text.text != "":
		emit_signal("connection", gui_answer_text.text)


func _on_paste_pressed():
	gui_answer_text.text = DisplayServer.clipboard_get()
	if gui_answer_text.text != "":
		emit_signal("connection", gui_answer_text.text)


func logy(lvl, msg):
	print(lvl, msg)
