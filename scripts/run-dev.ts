import { spawn } from 'child_process';

const bunProcess = spawn('bun', ['run', 'watch'], {
    stdio: 'inherit',
    shell: true
});

const cargoProcess = spawn('cargo', ['r'], {
    stdio: 'inherit',
    shell: true
});

bunProcess.on('error', (err) => {
    console.error('Failed to start bun process:', err);
});

cargoProcess.on('error', (err) => {
    console.error('Failed to start cargo process:', err);
});

process.on('SIGINT', () => {
    bunProcess.kill();
    cargoProcess.kill();
    process.exit();
});
