import * as monaco from 'monaco-editor/esm/vs/editor/editor.api';
// The editor

const container = document.getElementById('code');
const editor = monaco.editor.create(container!, {
    value: [
        'def fact(n):',
        '\tif n == 0:',
        '\t\treturn 1',
        '\telse:',
        '\t\treturn n * fact(n - 1)',
        '',
        'print(fact(5))'
    ].join('\n'),
	language: 'python'
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
    stderr.innerText = `exit code: ${response.exit_code}\n` + response.stderr;
}

export const run_code = () => {
    const code = editor.getValue();
    const payload = {"language": "Python", "code": code};
    console.log("Sending payload");

    const req = new XMLHttpRequest();
    req.addEventListener("load", code_result);
    req.open("POST", "/run-code", true);
    req.setRequestHeader('Content-Type', 'application/json');
    req.send(JSON.stringify(payload));
}

const run_code_btn = <HTMLButtonElement>document.getElementById('execute');
run_code_btn.addEventListener('click', run_code);
