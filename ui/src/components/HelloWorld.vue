<template>
    <b-container fluid>
        {{todos}} asdsad {{totalrows}}
    <p @click="remove(155)">remove</p>
    </b-container>
</template>

<script>
  export default {
    data() {
      return {
          data:[],
    }
    },
      methods:{
          remove (item) {
              this.$store.dispatch('RemoveTable',item);
          }
      },
    created() {
        this.$store.dispatch('FetchSummaryTables');
  },
      computed:{
          todos () { return this.$store.getters.tableSelectForm },
          totalrows(){ return this.$store.getters.totalTables }
      }
  }
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
  .myclass {
    background-color: black;
  }
</style>
