var hatao = ()=>{
    try {
        var div = document.getElementById('output');
        div.innerHTML = `
		<div class="terminalprompt">
			<span class="greenarrow">&nbsp;></span>
			<span class="cursor"></span>
			<span class="log"></span>
		</div>`;
		return ("done");
    } catch (e) {
        console.error(e);
    }
}
var bolo = (shbad)=>{
    try {		
        var div = document.createElement('div');
		div.className = "terminalprompt";
        div.innerHTML = `
			<span class="greenarrow">&nbsp;>&nbsp;</span>
			<span class="log">${shbad}</span>`;
        var terminal = document.getElementById('output');
        terminal.insertBefore(div, terminal.lastElementChild);
        return ("done");
    } catch (e) {
        console.error(e);
    }
}
var compiler = (code)=>{
    try {
		code = code.replace(/mana/g, "var");
		eval(code);
		return ("done");
    } catch (e) {
        console.error(e);
    }
}

var compile = ()=>{
    let editor = document.getElementById('editor');
    let code = editor.value;
    compiler(code);
}