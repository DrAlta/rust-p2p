extends Node

var clock = 0.0
# Create the two peers
var p1 = P2PLink.new("bob")
var p2 = WebRTCPeerConnection.new()
# And a negotiated channel for each each peer
#var ch1 = p1.create_data_channel("chat", {"id": 1, "negotiated": true})
var ch2 = p2.create_data_channel("main", {"negotiated": true, "id": 1})

func set_p2_sess_desc(data):
	print("conninging p2")
	p2.set_remote_description("offer", data.Offer)
	
func _ready():
	# Connect P1 session created to itself to set local description.
	#p1.connection.session_description_created.connect(test)#p1.set_local_description)
	#p1.session_description_created.connect(p1.set_local_description)
	# Connect P1 session and ICE created to p2 set remote description and candidates.

# this giving p1 sesssion desc to p2 is tthe problem
	#p1.connection.session_description_created.connect(p2.set_remote_description)
	p1.connect("offer_generated", set_p2_sess_desc)

	#p1.ice_candidate_created.connect(p2.add_ice_candidate)

	# Same for P2
	p2.session_description_created.connect(p2.set_local_description)
	p2.session_description_created.connect(p1.set_remote_description)
	p2.ice_candidate_created.connect(p1.add_ice_candidate)

	# Let P1 create the offer
	p1.create_offer()
	#p1.connection.create_offer()

func test(a, b:String):
	print(a, " created:", b.length())

	# Wait a second and send message from P1.
func foo():
	#print("#")
	p1.send("Hi from P1")
#	if ch1.get_ready_state() == WebRTCDataChannel.STATE_OPEN:
#		ch1.put_packet("Hi from P1".to_utf8_buffer())

	# Wait a second and send message from P2.
	#print('2')
	if ch2.get_ready_state() == WebRTCDataChannel.STATE_OPEN:
		ch2.put_packet("Hi from P2".to_utf8_buffer())

func _process(delta):
	clock += delta
	if clock > 2.5:
		foo()
		clock = 0.0
	# Poll connections
	p1.poll()
	p2.poll()

	# Check for messages
	if p1.get_ch_ready_state() == WebRTCDataChannel.STATE_OPEN and p1.get_available_packet_count() > 0:
		print("P1 received: ", p1.get_packet())
#	if ch1.get_ready_state() == ch1.STATE_OPEN and ch1.get_available_packet_count() > 0:
#		print("P1 received: ", ch1.get_packet().get_string_from_utf8())
	if ch2.get_ready_state() == ch2.STATE_OPEN and ch2.get_available_packet_count() > 0:
		print("P2 received: ", ch2.get_packet().get_string_from_utf8())
