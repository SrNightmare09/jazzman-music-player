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

	const album_sidebar = e.target.closest("#sidebar-album");

	if (album_sidebar) {
		showAlbumDetails(album_sidebar.innerText);
	}

	const playlist_sidebar = e.target.closest('#sidebar-playlist');

	if (playlist_sidebar) {
		console.log('hello');
		showPlaylistView();
	}
});

async function fetchArtistAlbums(artist) {
	var albums = await tauri.invoke('fetch', { selectQry: 'song_artwork', tableQry: 'songs', whereQry: 'song_artist', item: artist });
	albums = [...new Set(albums[0])];
	const main_view = document.getElementById('main-view');
	const playlist_view = document.getElementById('playlist-details');
	const album_details = document.getElementById('album-details');

	playlist_view.style.display = 'none';
	album_details.style.display = 'none';
	main_view.innerHTML = '';

	for (const key in albums) {
		let child = document.createElement('span');
		child.classList.add('grid-cell');
		child.setAttribute('id', 'grid-cell');
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

async function showAlbumDetails(album) { // hell(p)

	const scroll_view = document.getElementById('main-view-scroll');
	const album_details = document.getElementById('album-details');
	const album_name = document.getElementById('album-details-name');
	const artist_name = document.getElementById('album-details-artist');
	const album_artwork = document.getElementById('album-details-artwork');
	const song_list = document.getElementById('album-details-songs');
	const playlist_details = document.getElementById('playlist-details');
	const main_view = document.getElementById('main-view');

	// remove current artwork
	main_view.innerHTML = '';

	var songs = await tauri.invoke('fetch', { selectQry: 'song_name', tableQry: 'songs', whereQry: 'song_album', item: album });
	var artist = await tauri.invoke('fetch', { selectQry: 'song_artist', tableQry: 'songs', whereQry: 'song_album', item: album });
	var artwork = await tauri.invoke('fetch', { selectQry: 'song_artwork', tableQry: 'songs', whereQry: 'song_album', item: album });

	songs = [...new Set(songs[0])];
	artist = [...new Set(artist[0])];
	artwork = [...new Set(artwork[0])];

	playlist_details.style.display = 'none';
	scroll_view.style.display = 'none';
	album_details.style.display = 'block';
	album_name.innerText = album;
	artist_name.innerText = artist[0];
	album_artwork.style.backgroundImage = `url('${artwork}')`;
	song_list.innerHTML = '';

	for (let i = 0; i < songs.length; i++) {
		var e_song_wrapper = document.createElement('div');
		e_song_wrapper.setAttribute('class', 'album-details-album-song');
		e_song_wrapper.setAttribute('id', `song-no-${i + 1}`);
		song_list.appendChild(e_song_wrapper);

		let song_wrapper = document.getElementById(`song-no-${i + 1}`);

		let song_s_no = document.createElement('div');
		song_s_no.setAttribute('id', 'song-s-no');
		song_s_no.innerText = `${i + 1}.`;
		song_wrapper.appendChild(song_s_no);

		let song_name = document.createElement('div');
		song_name.setAttribute('id', 'song-name');
		song_name.innerText = songs[i];
		song_wrapper.appendChild(song_name);

		let song_like_status = document.createElement('div');
		song_like_status.setAttribute('id', 'song-like-status');
		song_like_status.innerText = 'L';
		song_wrapper.appendChild(song_like_status);

		let song_length = document.createElement('div');
		song_length.setAttribute('id', 'song-length');
		song_length.innerText = '0:00';
		song_wrapper.appendChild(song_length);
	}
}

async function showPlaylistView() {
	const playlist_view = document.getElementById('playlist-details');
	const album_details = document.getElementById('album-details');
	const main_view = document.getElementById('main-view');

	main_view.innerHTML = '';

	album_details.style.display = 'none';
	playlist_view.style.display = 'block';
}
