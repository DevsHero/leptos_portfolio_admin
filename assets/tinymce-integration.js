// Add this script to your index.html or create a separate JS file
(function() {
    // Store editor instances
    window.tinyMCEInstances = {};
    
    // Initialize TinyMCE
    window.init_tiny_mce = function(selector, content, onChangeCallback) {
      // Make sure TinyMCE is loaded
      if (!window.tinymce) {
        const script = document.createElement('script');
        script.src = 'https://cdnjs.cloudflare.com/ajax/libs/tinymce/6.3.1/tinymce.min.js';
        script.referrerpolicy = 'origin';
        script.onload = function() {
          initEditor(selector, content, onChangeCallback);
        };
        document.head.appendChild(script);
      } else {
        initEditor(selector, content, onChangeCallback);
      }
    };
  
    function initEditor(selector, content, onChangeCallback) {
      const editorId = selector.substring(1); // Remove the # from the selector
      
      tinymce.init({
        selector: selector,
        plugins: 'anchor autolink charmap codesample emoticons image link lists media searchreplace table visualblocks wordcount',
        toolbar: 'undo redo | blocks fontfamily fontsize | bold italic underline strikethrough | link image media table | align lineheight | numlist bullist indent outdent | emoticons charmap | removeformat',
        setup: function(editor) {
          editor.on('init', function() {
            // Set initial content
            editor.setContent(content);
            
            // Store the editor instance
            window.tinyMCEInstances[editorId] = editor;
          });
          
          editor.on('change', function() {
            const content = editor.getContent();
            onChangeCallback(content);
          });
        }
      });
    }
  
    // Get content from editor
    window.get_tiny_mce_content = function(editorId) {
      const editor = window.tinyMCEInstances[editorId];
      return editor ? editor.getContent() : '';
    };
  
    // Set content to editor
    window.set_tiny_mce_content = function(editorId, content) {
      const editor = window.tinyMCEInstances[editorId];
      if (editor && editor.getContent() !== content) {
        editor.setContent(content);
      }
    };
  
    // Destroy editor instance
    window.destroy_tiny_mce = function(editorId) {
      const editor = window.tinyMCEInstances[editorId];
      if (editor) {
        editor.destroy();
        delete window.tinyMCEInstances[editorId];
      }
    };
  })();