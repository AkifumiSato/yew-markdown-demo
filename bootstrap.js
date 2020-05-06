import './static/style/root.scss';

import("./pkg").then(module => {
  module.run_app();
});
