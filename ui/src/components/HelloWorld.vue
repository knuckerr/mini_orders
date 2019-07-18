<template>
      <b-container fluid>
          <b-row>
      <b-col md="6" class="my-1">
        <b-form-group label-cols-sm="3" label="Filter" class="mb-0">
          <b-input-group>
            <b-form-input v-model="filter" placeholder="Type to Search"></b-form-input>
            <b-input-group-append>
              <b-button :disabled="!filter" @click="filter = ''">Clear</b-button>
            </b-input-group-append>
          </b-input-group>
        </b-form-group>
      </b-col>
      <b-col md="6" class="my-1">
        <b-form-group label-cols-sm="3" label="Per page" class="mb-0">
          <b-form-select v-model="perPage" :options="pageOptions"></b-form-select>
        </b-form-group>
      </b-col>
    </b-row>
    <b-table
      show-empty
      stacked="md"
      :items="orders"
      :fields="fields"
      :current-page="currentPage"
      :per-page="perPage"
      :filter="filter"
      @filtered="onFiltered">

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
      </b-container>
</template>

<script>
import axios from 'axios';
export default {
    data() {
    return {
      orders: [],
        fields: {
          'table': {
            label: 'Name',
            sortable: true,
            formatter: 'RoomName'
          },
          item_total: {
            label: 'Item Total',
            sortable: true
          },
          total: {
            label: 'Total',
            sortable: true
          },
          last_order: {
            label: 'Last Order',
            sortable: true
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
      this.totalRows = this.orders.length
    },
  methods: {
    getOrders() {
      const path = 'http://localhost:5000/sumary_tables';
      axios.get(path)
        .then((res) => {
          this.orders = res.data;
        })
        .catch((error) => {
          // eslint-disable-next-line
          console.error(error);
        });
    },
      onFiltered(filteredItems) {
        // Trigger pagination to update the number of buttons/pages due to filtering
        this.totalRows = filteredItems.length
        this.currentPage = 1
      },
      RoomName(value){
          return `Room ${value.name}`
      }
},

  created() {
    this.getOrders();
  },
};
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
</style>
