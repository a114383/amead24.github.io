import Vue from 'vue'
import App from './App.vue'
import router from './router'
import store from './store'

// Compare this to 'Automatic Global Registration'
// import BaseIcon from '@/components/BaseIcon.vue'
// Vue.component('BaseIcon', BaseIcon)

// First: npm install --save vue-lodash lodash
import upperFirst from 'lodash/upperFirst'
import camelCase from 'lodash/camelCase'

const requireComponent = require.context(
  './components',
  false,
  /Base[A-Z]\w+\.(vue|js)$/
)

requireComponent.keys().forEach(fileName => {
  const componentConfig = requireComponent(fileName)

  const componentName = upperFirst(
    camelCase(fileName.replace(/^\.\/(.*)\.\w+$/, '$1'))
  )

  Vue.component(componentName, componentConfig.default || componentConfig)
})

Vue.config.productionTip = false

new Vue({
  router,
  store,
  render: h => h(App)
}).$mount('#app')
