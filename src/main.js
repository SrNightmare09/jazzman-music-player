function toggleSubOptions(id) {
  const subOptions = document.getElementById('playlistsOptions');
  if (id === 'playlists') {
      subOptions.style.display = subOptions.style.display === "none" ? "block" : "none";
  }
}

function getLibraryArtwork() {
  let a =[4, 2, 3, 1, 23, 12, 3];

  const main_view = document.getElementById('test');

  let child = document.createElement('span');
  child.classList.add('grid-cell');
  child.style.backgroundImage = 'url(assets/Cover.jpg)';

  main_view.appendChild(child);
}
