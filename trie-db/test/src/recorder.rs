// Copyright 2017, 2020 Parity Technologies
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Trie query recorder.

use hash_db::Hasher;
use memory_db::{HashKey, MemoryDB};
use reference_trie::{RefHasher, RefTrieDB, RefTrieDBMut};
use trie_db::{Record, Recorder, Trie, TrieMut};

#[test]
fn basic_recorder() {
	let mut basic = Recorder::new();

	let node1 = vec![1, 2, 3, 4];
	let node2 = vec![4, 5, 6, 7, 8, 9, 10];

	let (hash1, hash2) = (RefHasher::hash(&node1), RefHasher::hash(&node2));
	basic.record(&hash1, &node1, 0);
	basic.record(&hash2, &node2, 456);

	let record1 = Record { data: node1, hash: hash1, depth: 0 };

	let record2 = Record { data: node2, hash: hash2, depth: 456 };

	assert_eq!(basic.drain(), vec![record1, record2]);
}

#[test]
fn basic_recorder_min_depth() {
	let mut basic = Recorder::with_depth(400);

	let node1 = vec![1, 2, 3, 4];
	let node2 = vec![4, 5, 6, 7, 8, 9, 10];

	let hash1 = RefHasher::hash(&node1);
	let hash2 = RefHasher::hash(&node2);
	basic.record(&hash1, &node1, 0);
	basic.record(&hash2, &node2, 456);

	let records = basic.drain();

	assert_eq!(records.len(), 1);

	assert_eq!(records[0].clone(), Record { data: node2, hash: hash2, depth: 456 });
}

#[test]
fn trie_record() {
	let mut db = MemoryDB::<RefHasher, HashKey<_>, _>::default();
	let mut root = Default::default();
	{
		let mut x = RefTrieDBMut::new(&mut db, &mut root);

		x.insert(b"dog", b"cat").unwrap();
		x.insert(b"lunch", b"time").unwrap();
		x.insert(b"notdog", b"notcat").unwrap();
		x.insert(b"hotdog", b"hotcat").unwrap();
		x.insert(b"letter", b"confusion").unwrap();
		x.insert(b"insert", b"remove").unwrap();
		x.insert(b"pirate", b"aargh!").unwrap();
		x.insert(b"yo ho ho", b"and a bottle of rum").unwrap();
	}

	let trie = RefTrieDB::new(&db, &root).unwrap();
	let mut recorder = Recorder::new();

	trie.get_with(b"pirate", &mut recorder).unwrap().unwrap();

	let nodes: Vec<_> = recorder.drain().into_iter().map(|r| r.data).collect();
	assert_eq!(
		nodes,
		vec![
			vec![
				254, 192, 0, 128, 32, 27, 87, 5, 125, 163, 0, 90, 117, 142, 28, 67, 189, 82, 249,
				72, 103, 181, 28, 167, 216, 106, 191, 152, 9, 255, 42, 59, 75, 199, 172, 190, 128,
				227, 98, 5, 56, 103, 215, 106, 0, 144, 78, 159, 78, 163, 198, 13, 159, 226, 112,
				82, 132, 211, 79, 143, 4, 16, 109, 253, 182, 34, 196, 39, 13
			],
			vec![
				254, 1, 2, 52, 11, 105, 114, 97, 116, 101, 24, 97, 97, 114, 103, 104, 33, 112, 15,
				111, 32, 104, 111, 32, 104, 111, 76, 97, 110, 100, 32, 97, 32, 98, 111, 116, 116,
				108, 101, 32, 111, 102, 32, 114, 117, 109
			]
		]
	);

	trie.get_with(b"letter", &mut recorder).unwrap().unwrap();

	let nodes: Vec<_> = recorder.drain().into_iter().map(|r| r.data).collect();
	assert_eq!(
		nodes,
		vec![
			vec![
				254, 192, 0, 128, 32, 27, 87, 5, 125, 163, 0, 90, 117, 142, 28, 67, 189, 82, 249,
				72, 103, 181, 28, 167, 216, 106, 191, 152, 9, 255, 42, 59, 75, 199, 172, 190, 128,
				227, 98, 5, 56, 103, 215, 106, 0, 144, 78, 159, 78, 163, 198, 13, 159, 226, 112,
				82, 132, 211, 79, 143, 4, 16, 109, 253, 182, 34, 196, 39, 13
			],
			vec![
				254, 16, 83, 28, 5, 111, 103, 12, 99, 97, 116, 52, 11, 111, 116, 100, 111, 103, 24,
				104, 111, 116, 99, 97, 116, 52, 11, 110, 115, 101, 114, 116, 24, 114, 101, 109,
				111, 118, 101, 124, 254, 192, 0, 64, 10, 5, 116, 116, 101, 114, 36, 99, 111, 110,
				102, 117, 115, 105, 111, 110, 40, 8, 5, 110, 99, 104, 16, 116, 105, 109, 101, 52,
				11, 111, 116, 100, 111, 103, 24, 110, 111, 116, 99, 97, 116
			]
		]
	);
}
