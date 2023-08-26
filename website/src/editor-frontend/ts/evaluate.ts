import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
// The editor

const EXECUTION_INTERVAL = 1000;

const container = document.getElementById('code');
const editor = monaco.editor.create(container!, {
    value: [
        'local function fact(n)',
        '\tif n == 0 then return 1 end',
        '',
        '\treturn n * fact(n - 1)',
        'end',
        '',
        '',
        'print(fact(5))\n',
    ].join('\n'),
	language: 'lua',
  automaticLayout: false,
});

export function code_result(this: XMLHttpRequest) {
    if (this.readyState !== this.DONE || this.status !== 200) {
        return;
    }

    const response = JSON.parse(this.responseText);
    console.log(`Received response: ${this.responseText}`);
    const stdin = <HTMLDivElement>document.getElementById("stdout")
    stdin.innerText = response.stdout;
    const stderr = <HTMLDivElement>document.getElementById("stderr");
    stderr.innerText = response.stderr;
}

export const run_code = () => {
    const code = editor.getValue();
    const payload = {"language": "Lua", "code": code};
    console.log("Sending payload");

    const req = new XMLHttpRequest();
    req.addEventListener("load", code_result);
    req.open("POST", "/run-code", true);
    req.setRequestHeader('Content-Type', 'application/json');
    req.send(JSON.stringify(payload));
}

// We want to run the code when user stops typing.
// However, we want to run the code only once when
// users stops typing.
//
// We use two variables to achieve this:
// code_changed is set to true if the code changed
// in the last x seconds.
let code_changed = true;
let previous_code = editor.getValue();

setInterval(() => {
    console.log("Checking for changes");
    const current_code = editor.getValue();
    if (current_code === previous_code && code_changed) {
        console.log("Running code");
        run_code();
        code_changed = false;
    } else if (current_code !== previous_code) {
        console.log("Code changed");
        code_changed = true;
    }
    previous_code = current_code;
}, EXECUTION_INTERVAL);

// Resize the editor
// Preferably, we would like to use the
// 'automaticLayout' config option in the editor,
// but it only works when making the view bigger,
// not when shrinking it.
window.addEventListener("resize", function() {
    // make editor as small as possible
    editor.layout({ width: 0, height: 0 })

    // Resize when the frame refreshes
    window.requestAnimationFrame(() => {
        const parent = document.getElementById('code');
        editor.layout(parent?.getBoundingClientRect());
    });
});
