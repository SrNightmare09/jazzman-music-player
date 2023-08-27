const tauri = window.__TAURI__;

(function () {
	initialize();
})();

async function initialize() {

	await tauri.invoke('initialize', {});
}

function toggleSubOptions(id) {
	const element = document.getElementById(id + 'Options');
	if (element.style.display === 'none' || element.style.display === '') {
		element.style.display = 'block';
	} else {
		element.style.display = 'none';
	}
}

async function scan_music() {
	tauri.invoke('scan_music', {});
	getLibraryArtwork();
}

async function getLibraryArtwork() {
	const main_view = document.getElementById('main-view');

	// remove current artwork
	main_view.innerHTML = '';

	let artwork = await tauri.invoke('fetch', { selectQry: 'song_artwork', tableQry: 'songs', whereQry: 'song_artist', item: '%' });

	artwork = [...new Set(artwork[0])];

	artwork.forEach((dir) => {
		let child = document.createElement('span');
		child.classList.add('grid-cell');
		child.style.backgroundImage = `url('${dir}')`; //! TODO: find some way to enable directories with spaces
		main_view.appendChild(child);
	});

	getArtists();
	getAlbums();
}

async function getArtists() {
	const artist_list = document.getElementById('artistsOptions');

	artist_list.innerHTML = '';

	var artist = await tauri.invoke('fetch', { selectQry: 'song_artist', tableQry: 'songs', whereQry: 'song_artist', item: '%' });

	artist = [...new Set(artist[0])];;

	artist.forEach((song_artist) => {
		let child = document.createElement('div');
		child.classList.add('sidebar-item');
		child.setAttribute("id", "sidebar-artist");
		child.innerText = song_artist;
		artist_list.appendChild(child);
	});
}

async function getAlbums() {
	const album_list = document.getElementById('albumsOptions');

	album_list.innerHTML = '';

	var album = await tauri.invoke('fetch', { selectQry: 'song_album', tableQry: 'songs', whereQry: 'song_artist', item: '%' });

	album = [...new Set(album[0])];

	album.forEach((album_artist) => {
		let child = document.createElement('div');
		child.classList.add('sidebar-item');
		child.setAttribute("id", "sidebar-album");
		child.innerText = album_artist;
		album_list.appendChild(child);
	});
}

document.addEventListener('click', (e) => {
	const artist_sidebar = e.target.closest('#sidebar-artist');

	if (artist_sidebar) {
		fetchArtistAlbums(artist_sidebar.innerText);
	}
});

async function fetchArtistAlbums(artist) {
	var albums = await tauri.invoke('fetch', { selectQry: 'song_artwork', tableQry: 'songs', whereQry: 'song_artist', item: artist });
	albums = [...new Set(albums[0])];
	const main_view = document.getElementById('main-view');

	main_view.innerHTML = '';

	for (const key in albums) {
		let child = document.createElement('span');
		child.classList.add('grid-cell');
		child.style.backgroundImage = `url('${albums[key].trim()}')`;
		main_view.appendChild(child);
	}
}

async function create_playlist_button() {
	document.getElementById('playlist-name-box').value = '';
	document.getElementById('playlist-name-dialog-box').style.display = 'block';
}

document.getElementById('create-playlist-button').onclick = () => {
	const playlist_name = document.getElementById('playlist-name-box').value;
	const playlist_sidebar = document.getElementById('playlistsOptions');

	if (playlist_name == '') {
		document.getElementById('playlist-name-dialog-box').style.display = 'none';
		return;
	}

	let child = document.createElement('div');
	child.classList.add('sidebar-item');
	child.setAttribute('id', 'sidebar-playlist');
	child.innerText = playlist_name;
	playlist_sidebar.appendChild(child);

	document.getElementById('playlist-name-dialog-box').style.display = 'none';
}

