<template>
  <div class="container-fluid">
    <div class="row">
      <div class="col-md-2">
        <div class="input-group">
          <input
            class="form-control"
            v-model="newitem"
            placeholder="New item"
          />
          <div class="input-group-append">
            <button
              class="btn btn-outline-secondary"
              @click="additem()"
              type="button"
            >
              Add
            </button>
          </div>
        </div>
      </div>
      <div class="col-md-2">
        <div class="input-group">
            <input
              class="form-control"
              v-model="range.from"
              type="number"
              min="0"
              max="90"
              placeholder="From"
            />
            <input
              class="form-control"
              type="number"
              min="0"
              max="100"
              v-model="range.to"
              placeholder="To"
            />
          <div class="input-group-append">
            <button
              class="btn btn-outline-secondary"
              @click="new_range_items()"
              type="button"
            >
              Add
            </button>
          </div>
        </div>
      </div>
      <div class="col-md-2">
        <div class="input-group">
          <input class="form-control" v-model="filter" placeholder="Search" />
          <div class="input-group-append">
            <button
              class="btn btn-outline-secondary"
              :disabled="!filter"
              @click="filter = ''"
              type="button"
            >
              Clear
            </button>
          </div>
        </div>
      </div>
      <div class="col-md-2">
        <div class="input-group">
          <select class="form-control" v-model="action">
            <option v-for="action in actions" v-bind:key="action">
            {{action}} selected
            </option>
          </select>
          <div class="input-group-append">
            <button
              class="btn btn-outline-secondary"
              @click="selected_action()"
              type="button"
            >
              Action
            </button>
          </div>
        </div>
      </div>
      <div class="col-md-2">
        <div class="form-group">
          <select class="form-control" v-model="perPage">
            <option v-for="page in pageOptions" v-bind:key="page">
              {{ page }}
            </option>
          </select>
        </div>
      </div>
    </div>
    <b-table
      show-empty
      stacked="md"
      :items="tables"
      :fields="fields"
      :filter="filter"
      :current-page="currentPage"
      :per-page="perPage"
      @filtered="onFiltered"
    >
      <template slot="HEAD_select">
        <input type="checkbox" v-model="selectall" @click="Toggleall" />
      </template>

      <template slot="select" slot-scope="item">
        <input type="checkbox" v-model="selectitems" :value="item.item.id" />
      </template>

      <template slot="methods" slot-scope="item">
        <div class="row">
          <div class="col-sm-2">
        <button
          type="button"
          class="btn btn-sm btn-danger"
          @click="remove(item.item.id)"
        >
          Delete
        </button>
          </div>
          <div class="col-sm-2">
        <button
          type="button"
          class="btn btn-sm btn-primary"
          @click="clear(item.item.id)"
        >
        Clear
        </button>
          </div>
        </div>
      </template>
    </b-table>
    <b-row>
      <b-col md="6" class="my-1">
        <b-pagination
          v-model="currentPage"
          :total-rows="totalrows"
          :per-page="perPage"
          class="my-0"
        ></b-pagination>
      </b-col>
    </b-row>
    {{ selectitems }}
  </div>
</template>

<script>
export default {
  data() {
    return {
      range: {
        from: '',
        to: '',
      },
      selectitems: [],
      newitem: null,
      selectall: false,
      filter: '',
      totalrows: 0,
      action:null,
      actions:['delete','clear'],
      pageOptions: [5, 10, 15],
      fields: [
        'select',
        'methods',
        {key: 'name', label: 'Name', sortable: true},
        {key: 'item_total', label: 'Items', sortable: true},
        {key: 'total', label: 'Amount', sortable: true},
        {key: 'last_order', label: 'Last Order'},
      ],
      currentPage: 1,
      perPage: 10,
      sortBy: null,
    };
  },
  methods: {
    remove(item) {
      this.$store.dispatch('RemoveTable', item);
    },
    clear(item) {
      this.$store.dispatch('ClearTables',[item]);
    },
    additem() {
      this.$store.dispatch('CreateTable', this.newitem);
      this.newitem = '';
    },
    selected_action(){
      if(this.selectitems.length > 0){
        if(this.action == 'delete selected'){
          this.$store.dispatch('RemoveTables',this.selectitems);
        }else{
          this.$store.dispatch('ClearTables',this.selectitems);
        }
      }
      this.action = '';
    },
    new_range_items() {
      let from = parseInt(this.range.from);
      let to = parseInt(this.range.to);
      if (
        from > 0 &&
        to > 0 &&
        to > from
      ) {
        this.$store.dispatch('CreateTables',this.range);
        this.range.from = '';
        this.range.to = '';
      }
    },
    Toggleall() {
      let all = [];
      if (!this.selectall) {
        this.tables.forEach(function(data) {
          all.push(data.id);
        });
      }
      this.selectitems = all;
    },
    onFiltered(items) {
      this.totalrows = items.length;
      this.currentPage = 1;
    },
  },
  created() {
    this.$store.dispatch('FetchSummaryTables');
  },
  watch: {
    tables() {
      this.selectitems = [];
      this.selectall = false;
      this.totalrows = this.tables.length;
    },
  },
  computed: {
    tables() {
      return this.$store.getters.tableSelectForm;
    },
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.myclass {
  background-color: black;
}
</style>
