<template>
  <b-container fluid>
    <b-row>
      <b-col md="2" class="my-1">
        <b-form-group class="mb-0">
          <b-input-group>
            <b-form-input
              v-model="filter"
              placeholder="Type to Search"
            ></b-form-input>
            <b-input-group-append>
              <b-button :disabled="!filter" @click="filter = ''"
                >Clear</b-button
              >
            </b-input-group-append>
          </b-input-group>
        </b-form-group>
      </b-col>
      <b-col md="2" class="my-1">
        <b-form-group class="mb-0">
          <b-form-select
            v-model="perPage"
            :options="pageOptions"
          ></b-form-select>
        </b-form-group>
      </b-col>
      <div class="col-md-2 my-1">
        <div class="input-group mb-3">
          <input
            type="text"
            class="form-control"
            placeholder="new item"
            v-model="new_item"
          />
          <div class="input-group-prepend">
            <button
              class="btn btn-outline-secondary"
              @click="New_Table"
              type="button"
            >
              New Item
            </button>
          </div>
        </div>
      </div>
      <div class="col-md-2 my-1">
          <button class="btn btn-sm btn-danger" @click="Delete_All(selectitems)">Delete Selected</button>
     </div>
    </b-row>
    <b-table
      show-empty
      stacked="md"
      :items="tables"
      :fields="fields"
      :current-page="currentPage"
      :per-page="perPage"
      :filter="filter"
      @filtered="onFiltered"
    >
      <template slot="HEAD_id">
        <input type="checkbox" v-model="sellect_all" @click="Sellect" />
      </template>
      <template slot="id" slot-scope="data">
        <input type="checkbox" :value="data.item.id" v-model="selectitems" />
      </template>
      <template slot="action" slot-scope="data">
        <div class="row">
          <div class="col-sm-1">
            <button
              type="button"
              class="btn btn-danger btn-sm"
              @click="Delete_Table(data.item.id)"
            >
              Delete
            </button>
          </div>
        </div>
      </template>

      <template slot="table" slot-scope="row">
        {{ row.value}}
      </template>
    </b-table>
    <b-row>
      <b-col md="6" class="my-1">
        <b-pagination
          v-model="currentPage"
          :total-rows="totalRows"
          :per-page="perPage"
          class="my-0"
        ></b-pagination>
      </b-col>
    </b-row>

    <pre>{{selectitems}}</pre>
  </b-container>
</template>

<script>
  import axios from 'axios';
  export default {
    data() {
      return {
        orders: [],
        new_item: null,
        selectitems: [],
        sellect_all: false,
        fields: {
          id: {
            label: 'Select',
            sortable: false,
          },

          action: {
            label: 'Action',
            sortable: false,
          },
          name: {
            label: 'Name',
            sortable: true,
          },
          item_total: {
            label: 'Item Total',
            sortable: true,
          },
          total: {
            label: 'Total',
            sortable: true,
          },
          last_order: {
            label: 'Last Order',
            sortable: true,
          },
        },
        totalRows: 1,
        currentPage: 1,
        perPage: 15,
        pageOptions: [5, 10, 15],
        filter: null,
      };
    },
    mounted() {
      // Set the initial number of items
      this.totalRows = this.tables.length;
    },
    methods: {
      getOrders() {
        const path = 'http://localhost:5000/sumary_tables';
        axios
          .get(path)
          .then(res => {
            this.orders = res.data;
          })
          .catch(error => {
            // eslint-disable-next-line
            console.error(error);
          });
      },
      onFiltered(filteredItems) {
        // Trigger pagination to update the number of buttons/pages due to filtering
        this.totalRows = filteredItems.length;
        this.currentPage = 1;
      },
      Sellect() {
        this.selectitems = [];
        if (!this.sellect_all) {
          for (let i in this.tables) {
            this.selectitems.push(this.tables[i].id);
          }
        }
      },
      Delete_Table(id) {
        axios
          .post('http://localhost:5000/del_table', [id])
          .then(() => {
            this.orders = this.orders.filter(p => p.table.id != id);
          })
          .catch(error => {
            // eslint-disable-next-line
            console.error(error);
          });
      },
      Delete_All(selectitems) {
        if (selectitems.length > 0) {
          axios
            .post('http://localhost:5000/del_table', selectitems)
            .then(() => {
              this.orders = this.orders.filter(p => !selectitems.includes(p.table.id));
            });
        }
      },
      New_Table() {
        if (this.new_item != null) {
          axios
            .post('http://localhost:5000/new_table?name=' + this.new_item)
            .then(() => {
              this.getOrders();
              this.new_item = null;
            })
            .catch(error => {
              // eslint-disable-next-line
              console.error(error);
            });
        }
      },
    },
    computed: {
      tables: {
        get: function() {
          let tables = [];
          this.orders.forEach(item => {
            let table = item.table;
            tables.push({
              name: 'Room: ' + table.name,
              id: table.id,
              item_total: item.item_total,
              total: item.total,
              last_order: item.last_order,
              checked: false,
            });
          });
          return tables;
        },
      },
    },
    watch: {
      tables() {
        this.totalRows = this.tables.length;
        this.selectitems = [];
      },
    },

    created() {
      this.getOrders();
    },
  };
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
  .myclass {
    background-color: black;
  }
</style>
