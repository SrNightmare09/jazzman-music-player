const { invoke } = window.__TAURI__.tauri;

const btn = document.getElementById('fetch-button');

btn.onclick = () => {
  invoke('get_folders', {}).then((res) => {
    document.getElementById('box').innerText = res;
  })
}
