function code_result(this: XMLHttpRequest) {
    if (this.readyState !== this.DONE || this.status !== 200) {
        return;
    }

    const response = JSON.parse(this.responseText);
    console.log(`Received response: ${this.responseText}`);
    const stdin = <HTMLTextAreaElement>document.getElementById("stdout")
    stdin.value = response.stdout;
    const stderr = <HTMLTextAreaElement>document.getElementById("stderr");
    stderr.value = `exit code: ${response.exit_code}\n` + response.stderr;
}

const run_code = () => {
    const code_area = <HTMLTextAreaElement>document.getElementById("code");
    const code = code_area.value;
    const payload = {"language": "Python", "code": code};
    console.log("Sending payload");

    const req = new XMLHttpRequest();
    req.addEventListener("load", code_result);
    req.open("POST", "/run-code", true);
    req.setRequestHeader('Content-Type', 'application/json');
    req.send(JSON.stringify(payload));
}
