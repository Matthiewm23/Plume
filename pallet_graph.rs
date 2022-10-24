#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use std::collections::HashMap;

#[frame_support::pallet]
pub mod pallet {

    #[pallet::config]
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
    pub struct Graph<VId, E=(), V=()> {
        vertices: HashMap<VId, V>, 
        adjacency: HashMap<VId, Vec<(VId,E)>>,
    }

	#[pallet::storage]
	#[pallet::getter(fn blog_posts)]
	pub(super) type Graph<VId, E=(), V=()> = StorageMap<_, Twox64Concat, T::Hash, Graph<VId, E=(), V=()>>;



    #[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        Graphcreated(),
        Vertexadded(VId, V),
        Edgeadded(VId, VId,E),
        Verticesadded(VId),
        Underectededgeadded(VId,VId,E),
	}

    #[pallet::call]
    impl<VId, E,V> Graph<VId, E,V> where
    VId: Eq + Hash,
    V: Hash,
    {
        #[pallet::weight(10000)]
		#[transactional]
        pub fn new() -> Graph<VId, E,V>{
            Graph {vertices: HashMap::new(), adjacency: HashMa::new() }
            Self::deposit_event(Event::Graphcreated());
        }

		#[pallet::weight(10000)]
		#[transactional]
        pub fn push_vertex(self: &mut Graph<VId, E,V>, vid: VId, vertex: V) {
            self.vertices.insert(vid, vertex);
            Self::deposit_event(Event::Vertexadded(vid,vertex));

        }

		#[pallet::weight(10000)]
		#[transactional]
        pub fn push_edge(self: &mut Graph<VId, E,V>, from: VId, to: VId, edge: E) {
            let adjacent_to_from = self.adjacency.entry(from).or_default(); // fetch value "from" in the hash map or create it if does not exist.
            adjacent_to_from.push((to, edge);)
            Self::deposit_event(Event::Edgeadded(from,to,edge));
        }
    
    }

    #[pallet::call]
    impl<VId, E,V> Graph<VId, E , ()> where
    VId: Eq + Hash,
    {
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn push_vid(self:&mut Self, vid: VId){
            self.vertices.insert(vid,());
            Self::deposit_event(Event::Verticesadded(vid));

        }
    
    }

    #[pallet::call]
    impl<VId, E,V> Graph<VId, E ,V> where
    VId: Eq + Hash + Clone,
    V: Hash,
    E:Clone,
    {
		#[pallet::weight(10000)]
		#[transactional]
        pub fn push_undirected_edge(
            self:&mut Self,
            from: VId,
            to: VId,
            edge:E
        ) {
            self.push_edge(from.clone(), to.clone(), edge.clone());
            self.push_edge(to, from, edge);
            Self::deposit_event(Event::Underectededgeadded(from, to, edge));

        }
    
    }

}
    
    
    // Exemple
    // Graph<VId = &str , E = Direction> {
    //     vertices:{
    //         "A":(),
    //         "E":()
    //     },
    //     adjacency: {
    //         "A": [("E", Right)]
    //     }
    // }





