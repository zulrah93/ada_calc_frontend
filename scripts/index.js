$(document).ready(function(){

    hljs.highlightAll(); // Ensure script tag is on the HTML side; used to enable syntax highlighting on the input

    $('#verbose_flag').change(function() {
        if(this.checked) {
            $('#verbose').prop('value', 'true');
        }
        else {
            $('#verbose').prop('value', 'false');
        }
    });

    $('#graph_flag').change(function() {
        if(this.checked) {
            $('#graph').prop('value', 'true');
        }
        else {
            $('#graph').prop('value', 'false');
        }
    });

    // Always update the json hidden view using the text from the json editor in real time -- ⚠️ might optimize if it impacts browser performance
    $('#json_editor').bind('click keypress mousedown', function() {
          $('#json_view').text($('#json_editor').text());
    });

});