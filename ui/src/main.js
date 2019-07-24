import 'bootstrap/dist/css/bootstrap.css';
import BootstrapVue from 'bootstrap-vue';
import Vue from 'vue';
import App from './App.vue';
import router from './router';
import { createStore } from './store'

Vue.use(BootstrapVue);

Vue.config.productionTip = false;
const store = createStore();

new Vue({
  router,
  store,
  render: h => h(App),
}).$mount('#app');
