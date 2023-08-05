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

	// remove repeated items
	let artwork = await tauri.invoke('fetch_item', { item: 'song_artwork' });

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

	let artist = await tauri.invoke('fetch_item', { item: 'song_artist' });

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

	let album = await tauri.invoke('fetch_item', { item: 'song_album' });

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

document.addEventListener('click', (e) => {
	const album_sidebar = e.target.closest("#sidebar-album");

	if (album_sidebar) {
		showAlbumDetails(album_sidebar.innerText);
	}
});

async function showAlbumDetails(album) { // hell(p)

	const scroll_view = document.getElementById('main-view-scroll');
	const album_details = document.getElementById('album-details');
	const album_name = document.getElementById('album-details-name');
	const artist_name = document.getElementById('album-details-artist');
	const album_artwork = document.getElementById('album-details-artwork');
	const song_list = document.getElementById('album-details-songs');

	const songs = await tauri.invoke('fetch_one_to_one', { selectQry: 'song_name', whereQry: 'song_album', query: album });
	const artist = [...new Set(await tauri.invoke('fetch_one_to_one', { selectQry: 'song_artist', whereQry: 'song_album', query: album }))];
	const artwork = [...new Set(await tauri.invoke('fetch_one_to_one', { selectQry: 'song_artwork', whereQry: 'song_album', query: album }))];

	scroll_view.style.display = 'none';
	album_details.style.display = 'block';
	album_name.innerText = album;
	artist_name.innerText = artist[0];
	album_artwork.style.backgroundImage = `url('${artwork}')`;
	song_list.innerHTML = '';

	let e_song_wrapper = document.createElement('div');
	e_song_wrapper.setAttribute('id', 'album-details-album-song');
	song_list.appendChild(e_song_wrapper);

	let song_wrapper = document.getElementById('album-details-album-song');
	let song_s_no = document.createElement('div');
	song_s_no.setAttribute('id', 'song-s-no');
	song_s_no.innerText = '69';

	let song_name = document.createElement('div');
	song_name.setAttribute('id', 'song-name');
	song_name.innerText = songs[0];

	let song_like_status = document.createElement('div');
	song_like_status.setAttribute('id', 'song-like-status');
	song_like_status.innerText = 'L';

	let song_length = document.createElement('div');
	song_length.setAttribute('id', 'song-length');
	song_length.innerText = '0:00';

	song_wrapper.appendChild(song_s_no);
	song_wrapper.appendChild(song_name);
	song_wrapper.appendChild(song_like_status);
	song_wrapper.appendChild(song_length);

}

async function fetchArtistAlbums(artist) {
	const albums = await tauri.invoke('fetch_specific_artwork', { query: artist });
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
