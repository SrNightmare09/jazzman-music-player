const tauri = window.__TAURI__;

function toggleSubOptions(id) {
	const element = document.getElementById(id + 'Options');
    if (element.style.display === 'none' || element.style.display === '') {
        element.style.display = 'block';
    } else {
        element.style.display = 'none';
    }
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
	getAlbums();
}

async function getArtists() {
	const artist_list = document.getElementById('artistsOptions');

	artist_list.innerHTML = '';

	let artist = [...new Set(await tauri.invoke('fetch_item', { item: 'song_artist' }))].sort();

	artist.forEach((song_artist) => {
		let child = document.createElement('div');
		child.classList.add('sidebar-item');
		child.innerText = song_artist;
		artist_list.appendChild(child);
	});
}

async function getAlbums() {
	const album_list = document.getElementById('albumsOptions');

	album_list.innerHTML = '';

	let album = [...new Set(await tauri.invoke('fetch_item', { item: 'song_album' }))].sort();

	album.forEach((album_artist) => {
		let child = document.createElement('div');
		child.classList.add('sidebar-item');
		child.innerText = album_artist;
		album_list.appendChild(child);
	});
}
