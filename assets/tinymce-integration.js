(function() {
    window.tinyMCEInstances = {};
    window.init_tiny_mce = function(selector, content, onChangeCallback) {
      if (!window.tinymce) {
        const script = document.createElement('script');
        script.src =  'https://cdnjs.cloudflare.com/ajax/libs/tinymce/6.8.5/tinymce.min.js';
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
      const editorId = selector.substring(1); 
      console.log(selector)
      console.log(editorId)
      tinymce.init({
        selector: selector,
        plugins: 'anchor autolink charmap codesample emoticons image link lists media searchreplace table visualblocks wordcount',
        toolbar: 'undo redo | blocks fontfamily fontsize | bold italic underline strikethrough | link image media table | align lineheight | numlist bullist indent outdent | emoticons charmap | removeformat',
        branding: false,
      
        promotion: false,
        setup: function(editor) {
          editor.on('init', function() {
            editor.setContent(content);
            window.tinyMCEInstances[editorId] = editor;
          });
          
          editor.on('change', function() {
            const content = editor.getContent();
            onChangeCallback(content);
          });
        }
      });
    }

    window.get_tiny_mce_content = function(editorId) {
      const editor = window.tinyMCEInstances[editorId];
      return editor ? editor.getContent() : '';
    };
  
    window.set_tiny_mce_content = function(editorId, content) {
      const editor = window.tinyMCEInstances[editorId];
      if (editor && editor.getContent() !== content) {
        editor.setContent(content);
      }
    };

    window.destroy_tiny_mce = function(editorId) {
      const editor = window.tinyMCEInstances[editorId];
      if (editor) {
        editor.destroy();
        delete window.tinyMCEInstances[editorId];
      }
    };
  })();