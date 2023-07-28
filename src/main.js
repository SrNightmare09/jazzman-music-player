const tauri = window.__TAURI__;

function toggleSubOptions(id) {
	const subOptions = document.getElementById('playlistsOptions');
	if (id === 'playlists') {
		subOptions.style.display = subOptions.style.display === "none" ? "block" : "none";
	}
}

async function getLibraryArtwork() {
	const main_view = document.getElementById('main-view');

	let artwork = [...new Set(await rustfun())];

	artwork.forEach((dir) => {
		let child = document.createElement('span');
		child.classList.add('grid-cell');
		child.style.backgroundImage = `url('${dir}')`; // find some way to enable directories with spaces
		main_view.appendChild(child);
	});
}

async function rustfun() {
	try {
		const result = await tauri.invoke('fetch_item', { item: 'song_artwork' });
		return result;
	} catch (error) {
		console.error('Error calling Rust function:', error);
	}
}
