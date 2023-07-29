const tauri = window.__TAURI__;

function toggleSubOptions(id) {
	const subOptions = document.getElementById('playlistsOptions');
	if (id === 'playlists') {
		subOptions.style.display = subOptions.style.display === "none" ? "block" : "none";
	}
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
}

function scan_music() {
	tauri.invoke('scan_music', {});
	getLibraryArtwork();
}
