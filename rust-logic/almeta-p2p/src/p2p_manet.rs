//! `newbie` sends request to connect to 'oldbie'
//! `oldbie` send it's hotlist to 'newbie'
//! `newbie` send the distance to the node on `oldbie`'s hotlist to 'oldbie'
//! `oldbie` add the distant to `newbie` to the retuned distances to the 
//! hotcakes and checks if it is smaller than `oldbie`'s distance the hotcake
//! 

use std::collections::HashSet;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Spam {

}
impl Spam{
    /// We assume peers are fully charged to their maximum energy level at the 
    /// beginning, so complimentary energy level equals the energy peer has 
    /// consumed so far. For convenience, the consumed and residual energy is 
    /// expressed as a fraction of the maximum energy level.
    pub fn complimentary_energy_level(&self) -> f64 {
        todo!()
    }
    pub fn neighbors(&self)-> &HashSet::<Spam> {
        todo!()
    }
    pub fn hotlist(&self) -> &HashSet::<Spam> {
        todo!()
    }
     
}
/// equation 4.3
/// The cost function used in the heuristic is similar to Equation 4.1, except
/// that only the distance to a subset of P is determined
fn four_three(i: &Spam) -> f64 {
    let alpha = 1.0;
    let y: f64 = i.neighbors().iter().map(|neighbor| neighbor.complimentary_energy_level()).sum();
    let x: f64 =  i.hotlist().iter().map(|hotstuff| stretch(i, hotstuff)).sum();
    alpha * y + x
}

/// We define the stretch as the ratio between the total number of 
    /// physical hops traversed and the shortest physical distance possible. 
    /// A peer has the minimum stretch of 1 to its overlay neighbors.
    /// returns the stretch from peer i to peer j
    pub fn stretch(i: &Spam, j: &Spam) -> f64 {
        todo!()
    }