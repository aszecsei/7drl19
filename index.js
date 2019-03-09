import './app.css';
import module from './Cargo.toml';

let game = module.run();

function step(timestamp) {
    game.update(timestamp);
    window.requestAnimationFrame(step);
}

window.requestAnimationFrame(step);

document.addEventListener('keydown', (e) => {
    console.log("down: " + e.code);
    game.key_down(e.code);
    e.preventDefault();
});
document.addEventListener('keyup', (e) => {
    console.log("up: " + e.code);
    game.key_up(e.code);
    e.preventDefault();
});

