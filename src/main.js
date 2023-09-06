const tauri = window.__TAURI__;
const homeView = document.getElementById('home-view');
const artistView = document.getElementById('artist-view');
const playlistView = document.getElementById('playlist-view');

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

async function scanMusic() {
	tauri.invoke('scan_music', {});
	getLibraryArtwork();
}

async function getLibraryArtwork() {

	// remove current artwork
	homeView.innerHTML = '';
	artistView.style.display = 'none';
	homeView.style.display = 'grid';

	let artwork = await tauri.invoke('fetch', { selectQry: 'song_artwork', tableQry: 'songs', whereQry: 'song_artist', item: '%' });

	artwork = [...new Set(artwork[0])];

	artwork.forEach((dir) => {
		let child = document.createElement('span');
		child.classList.add('grid-cell');
		child.style.backgroundImage = `url('${dir}')`; //! TODO: find some way to enable directories with spaces
		homeView.appendChild(child);
	});

	getArtists();
}

async function getArtists() {
	const artistList = document.getElementById('artistsOptions');

	artistList.innerHTML = '';

	var artist = await tauri.invoke('fetch', { selectQry: 'song_artist', tableQry: 'songs', whereQry: 'song_artist', item: '%' });

	artist = [...new Set(artist[0])];;

	artist.forEach((song_artist) => {
		let child = document.createElement('div');
		child.classList.add('sidebar-item');
		child.setAttribute("id", "sidebar-artist");
		child.innerText = song_artist;
		artistList.appendChild(child);
	});
}

document.addEventListener('click', (e) => {
	const artistSidebar = e.target.closest('#sidebar-artist');

	if (artistSidebar) {
		fetchArtistAlbums(artistSidebar.innerText);
	}

	const playlist = e.target.closest('#sidebar-playlist');

	if (playlist) {
		showPlaylistView(playlist.innerText);
	}
});

async function fetchArtistAlbums(artist) {
	var albums = await tauri.invoke('fetch', { selectQry: 'song_artwork', tableQry: 'songs', whereQry: 'song_artist', item: artist });
	albums = [...new Set(albums[0])];

	artistView.innerHTML = '';
	homeView.style.display = 'none';
	artistView.style.display = 'grid';

	for (const key in albums) {
		let child = document.createElement('span');
		child.classList.add('grid-cell');
		child.setAttribute('id', 'grid-cell');
		child.style.backgroundImage = `url('${albums[key].trim()}')`;
		artistView.appendChild(child);
	}
}

async function createPlaylistButton() {
	document.getElementById('playlist-name-box').value = '';
	document.getElementById('playlist-name-dialog-box').style.display = 'block';
}

document.getElementById('create-playlist-button').onclick = () => {
	const playlistName = document.getElementById('playlist-name-box').value;
	const playlistSidebar = document.getElementById('playlistsOptions');

	if (playlistName == '') {
		document.getElementById('playlist-name-dialog-box').style.display = 'none';
		return;
	}

	let child = document.createElement('div');
	child.classList.add('sidebar-item');
	child.setAttribute('id', 'sidebar-playlist');
	child.innerText = playlistName;
	playlistSidebar.appendChild(child);

	document.getElementById('playlist-name-dialog-box').style.display = 'none';
}

async function showPlaylistView(playlistName) {
	homeView.style.display = 'none';
	artistView.style.display = 'none';
	playlistView.style.display = 'initial';

	document.getElementById('playlist-name').innerText = playlistName;
}
