//! the Unrouting Algoithum. Instead of finding the best path for reaching a node we find the woost path for advoiding node.
//! * bellman-ford but if you loose the link you where routing to S then you completely foret about S and broadcast to all your neaghbors that you no loger know how to get to S
//! * when your receive an anouncemt that someone nolonger knows how to get to S you check if you where routing to S though that link
//! ** if you wwre outing though that link you re broadcast to all your neighbors that the route to S is down
//! ** if you weren't routign thou the link then you do an anouncement of your distance to S back on the link
//! * if some race cnditin happens aloop could be formed
//! ** when normal updating you routing table if the distance is greater than the old distance then you initalisea trace route
//! ** so when you get it back you tac youself own and anounce that route in a DistanceIncrease packet
//! *** if you get a DistaneIncrease packet to S from a link you are routing to S though then check if you are on it
//! ***** if you are on it then broadcast you don't known a route to S lke in a link failure 
//! ***** if you aren't update your distance and tac your self onto the end of the route and broadcast it in a DistanceIncrease to your other neihbors


/* this stuff was wrong

//! **3* if you get a traceroute check if you are the node at your distance to S
//! **** if you are then ignor it
//! **** if you are search it for self
//! ***** if you find yourself then broadast to your neighbors that you don't know how to reach S to all your peers just like in a link failure
//! /* this is equaant
//! *** if you get a route trace check if you are on it
//! **** if you are then check if you are still routing to S though thte node ahead of you in the trace
//! ***** if you are and the distance doeesn't match anounce you don't know how to reach S to all your peers just like in a link failure
//! */
*/

//Can you add bellman-ford routing algorythom to this code?

