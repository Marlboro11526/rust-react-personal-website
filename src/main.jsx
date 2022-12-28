import React, { Suspense } from 'react'
import ReactDOM from 'react-dom/client'

import './index.css'

const root = document.getElementById('root');
const modules = import.meta.glob('./pages/**/*.jsx')

function App() {
  const pageData = JSON.parse(root.dataset.page);
  const Component = React.lazy(modules[`./pages/${pageData.component}.jsx`] || modules[`./pages/${pageData.component}/index.jsx`])

  return (
    <Suspense fallback={null}>
      <Component {...pageData.props} />
    </Suspense>
  )
}

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
