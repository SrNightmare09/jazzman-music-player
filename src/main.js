const tauri = window.__TAURI__;

const home_view = document.getElementById('home-view');
const artist_view = document.getElementById('artist-view');
const playlist_view = document.getElementById('playlist-view');

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

	// remove current artwork
	home_view.innerHTML = '';
	artist_view.style.display = 'none';
	home_view.style.display = 'grid';

	let artwork = await tauri.invoke('fetch', { selectQry: 'song_artwork', tableQry: 'songs', whereQry: 'song_artist', item: '%' });

	artwork = [...new Set(artwork[0])];

	artwork.forEach((dir) => {
		let child = document.createElement('span');
		child.classList.add('grid-cell');
		child.style.backgroundImage = `url('${dir}')`; //! TODO: find some way to enable directories with spaces
		home_view.appendChild(child);
	});

	getArtists();
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

document.addEventListener('click', (e) => {
	const artist_sidebar = e.target.closest('#sidebar-artist');

	if (artist_sidebar) {
		fetchArtistAlbums(artist_sidebar.innerText);
	}

	const playlist = e.target.closest('#sidebar-playlist');

	if (playlist) {
		playlistView(playlist.innerText);
	}
});


async function navigateHome() {
	getLibraryArtwork();
}

async function fetchArtistAlbums(artist) {
	var albums = await tauri.invoke('fetch', { selectQry: 'song_artwork', tableQry: 'songs', whereQry: 'song_artist', item: artist });
	albums = [...new Set(albums[0])];

	artist_view.innerHTML = '';
	home_view.style.display = 'none';
	artist_view.style.display = 'grid';

	for (const key in albums) {
		let child = document.createElement('span');
		child.classList.add('grid-cell');
		child.setAttribute('id', 'grid-cell');
		child.style.backgroundImage = `url('${albums[key].trim()}')`;
		artist_view.appendChild(child);
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

async function playlistView(playlist_name) {
	home_view.style.display = 'none';
	artist_view.style.display = 'none';
	playlist_view.style.display = 'initial';
    
	document.getElementById('playlist-name').innerText = playlist_name;
}

async function showPlaylistView() {
	const playlist_view = document.getElementById('playlist-details');
	const album_details = document.getElementById('album-details');
	const main_view = document.getElementById('main-view');

	main_view.innerHTML = '';

	album_details.style.display = 'none';
	playlist_view.style.display = 'block';
}
