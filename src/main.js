const tauri = window.__TAURI__;

function toggleSubOptions(id) {
	const element = document.getElementById(id + 'Options');
	element.style.display = element.style.display === 'none' ? 'block' : 'none';
}

function scan_music() {
	tauri.invoke('scan_music', {});
	getLibraryArtwork();
}

async function getLibraryArtwork() {
	const main_view = document.getElementById('main-view');

	// remove current artwork
	main_view.innerHTML = '';

	// remove repeated items
	let artwork = [...new Set(await tauri.invoke('fetch_item', { item: 'song_artwork' }))];

	artwork.forEach((dir) => {
		let child = document.createElement('span');
		child.classList.add('grid-cell');
		child.style.backgroundImage = `url('${dir}')`; // find some way to enable directories with spaces
		main_view.appendChild(child);
	});

	getArtists();
}

async function getArtists() {
	const artist_list = document.getElementById('artistsOptions');

	let artist = [...new Set(await tauri.invoke('fetch_item', { item: 'song_artist' }))];

	artist.forEach((song_artist) => {
		let child = document.createElement('div');
		child.classList.add('sidebar-item');
		child.innerText = song_artist;
		artist_list.appendChild(child);
	});
}

