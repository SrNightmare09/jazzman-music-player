function toggleSubOptions(id) {
	const subOptions = document.getElementById('playlistsOptions');
	if (id === 'playlists') {
		subOptions.style.display = subOptions.style.display === "none" ? "block" : "none";
	}
}

function getLibraryArtwork() {
	const main_view = document.getElementById('test');

	let child = document.createElement('span');
	child.classList.add('grid-cell');
	child.style.backgroundImage = 'url(assets/Cover.jpg)';

	main_view.appendChild(child);
}

async function rustfun() {
	try {
		const result = await window.__TAURI__.invoke('fetch_item', {});
		let albums = [];
		for (let i = 0; i < result.length(); i++) {
			let info = {
				name: result[i].name,
				artwork: result[i].artwork
			};
			albums.push(info);
		}
	} catch (error) {
		console.error('Error calling Rust function:', error);
	}
}
