#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::Currency, inherent::Vec, sp_runtime::traits::Hash, transactional, traits::ExistenceRequirement};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type Currency: Currency<Self::AccountId>;

		#[pallet::constant]
        type QuestionMinBytes: Get<u32>;

        #[pallet::constant]
        type QuestionMaxBytes: Get<u32>;

        #[pallet::constant]
        type SubjectMinBytes: Get<u32>;

        #[pallet::constant]
        type SubjectMaxBytes: Get<u32>; 

        #[pallet::constant]
        type PossibilityMinBytes: Get<u32>;

        #[pallet::constant]
        type PossibilityMaxBytes: Get<u32>; 
	}	

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Question<T: Config> {
        pub content: Vec<u8>,
        pub author: <T as frame_system::Config>::AccountId,
	}

	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Subject<T: Config> {
        pub content: Vec<u8>,
        pub question_id: T::Hash,
        pub author: <T as frame_system::Config>::AccountId,
	}

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Possibility<T: Config> {
        pub content: Vec<u8>,
        pub subject_id: T::Hash,
        pub author: <T as frame_system::Config>::AccountId,
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);


	#[pallet::storage]
	#[pallet::getter(fn something)]

	pub type Something<T> = StorageValue<_, u32>;




	/// Storage Map for Question by questionid (Hash) to a question
	#[pallet::storage]
	#[pallet::getter(fn question)]
	pub(super) type Question<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Question<T>>;

	/// Storage Map from question id (Hash) to a list of subject for this question
	#[pallet::storage]
	#[pallet::getter(fn subject)]
	pub(super) type Subject<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Vec<Subject<T>>>;

    /// Storage Map from subject id (Hash) to a list of possibility for this subject
	#[pallet::storage]
	#[pallet::getter(fn possibility)]
	pub(super) type Possibility<T: Config> = StorageMap<_, Twox64Concat, T::Hash, Vec<Possibility<T>>>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {

		SomethingStored(u32, T::AccountId),
		QuestionCreated(Vec<u8>, T::AccountId, T::Hash),
		SubjectCreated(Vec<u8>, T::AccountId, T::Hash),
        PossibilityCreated(Vec<u8>, T::AccountId, T::Hash),
	}

	#[pallet::error]
	pub enum Error<T> {
        NoneValue,
        StorageOverflow,
        QuestionNotEnoughBytes, 
        QuestionTooManyBytes, 
        SubjectNotEnoughBytes,
        SubjectTooManyBytes,
        PossibilityNotEnoughBytes,
        PossibilityTooManyBytes,
        QuestionNotFound,
     
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {


		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => return Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}

		#[pallet::weight(10000)]
		#[transactional]
		pub fn create_question(origin: OriginFor<T>, content: Vec<u8>) -> DispatchResult {
				let author = ensure_signed(origin)?;

				ensure!(
						(content.len() as u32) > T::QuestionMinBytes::get(),
						<Error<T>>::QuestionNotEnoughBytes
				);

				ensure!(
						(content.len() as u32) < T::QuestionMaxBytes::get(),
						<Error<T>>::QuestionTooManyBytes
				);

				let question = Question { content: content.clone(), author: author.clone() };

				let question_id = T::Hashing::hash_of(&question);

				<Question<T>>::insert(question_id, question);

				let subjects_vec: Vec<Subject<T>> = Vec::new();

				<Subject<T>>::insert(question_id, subjects_vec);

				Self::deposit_event(Event::QuestionCreated(content, author, question_id));

				Ok(())
		}
	
		#[pallet::weight(5000)]
		pub fn create_subject(
				origin: OriginFor<T>,
				content: Vec<u8>,
				question_id: T::Hash,
		) -> DispatchResult {
				let comment_author = ensure_signed(origin)?;
		
				ensure!(
						(content.len() as u32) > T::QuestionMinBytes::get(),
						<Error<T>>::SubjectNotEnoughBytes
				);
		
				ensure!(
						(content.len() as u32) < T::QuestionMaxBytes::get(),
						<Error<T>>::SubjectTooManyBytes
				);
		
				let subject = Subject {
						author: comment_author.clone(),
						content: content.clone(),
						question_id: blog_post_id.clone(),
				};
		
				<Subject<T>>::mutate(question_id, |comments| match comments {
						None => Err(()),
						Some(vec) => {
								vec.push(subject);
								Ok(())
						},
				})
				.map_err(|_| <Error<T>>::QuestionNotFound)?;
		
				Self::deposit_event(Event::SubjectCreated(
						content,
						comment_author,
						question_id,
				));
		
				Ok(())
		}

        #[pallet::weight(5000)]
		pub fn create_possibility(
				origin: OriginFor<T>,
				content: Vec<u8>,
				subject_id: T::Hash,
		) -> DispatchResult {
				let comment_author = ensure_signed(origin)?;
		
				ensure!(
						(content.len() as u32) > T::QuestionMinBytes::get(),
						<Error<T>>::PossibilityNotEnoughBytes
				);
		
				ensure!(
						(content.len() as u32) < T::QuestionMaxBytes::get(),
						<Error<T>>::PossiblityTooManyBytes
				);
		
				let possibility = Possibility {
						author: comment_author.clone(),
						content: content.clone(),
						subject_id: blog_post_id.clone(),
				};
		
				<Possibility<T>>::mutate(subject_id, |comments| match comments {
						None => Err(()),
						Some(vec) => {
								vec.push(possibility);
								Ok(())
						},
				})
				.map_err(|_| <Error<T>>::QuestionNotFound)?;
		
				Self::deposit_event(Event::PossibilityCreated(
						content,
						comment_author,
						subject_id,
				));
		
				Ok(())
		}


	}
}



