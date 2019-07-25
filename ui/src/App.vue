<template>
  <div id="app" class="wrapper">
    <Navigation></Navigation>
    <router-view id="route" />
  </div>
</template>

<style>
#route {
    margin-top: 55px;
}
</style>
<script>
import navi from './components/navi'
export default {
  name: 'app',
  components: {
    'Navigation': navi
  }
}
</script>
