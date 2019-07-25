<template>
    <div class="container-fluid">
    <div class="row">
        <div class="col-xs-4">
            <div class="input-group">
                <input class="form-control" v-model="newitem" placeholder="New item">
                <div class="input-group-append">
                    <button class="btn btn-outline-secondary" @click="additem()" type="button">Button</button>
                </div>
            </div>
        </div>

    </div>
        <b-table
            show-empty
            stacked="md"
            :items="todos"
            :fields="fields"
            :current-page="currentPage"
            :per-page="perPage"
            >
            <template slot="HEAD_select">
                <input type="checkbox" v-model="selectall" @click="Toggleall">
            </template>

            <template slot="select" slot-scope="item">
                <input type="checkbox" v-model="selectitems" :value=item.item.id>
            </template>

            <template slot="method" slot-scope="item">
                <button type="button" class="btn btn-sm btn-danger" @click="remove(item.item.id)">Delete</button>
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
        {{selectitems}}
    </div>
</template>

<script>
export default {
    data() {
        return {
            selectitems: [],
            newitem: null,
            selectall:false,
            fields: [
                'select',
                'method',
                { key: 'name', label:'Name', sortable: true},
                { key: 'item_total', label: 'Items', sortable: true},
                { key: 'total', label: 'Amount',sortable:true },
                { key: 'last_order', label: 'Last Order' }
            ],
            currentPage: 1,
            perPage: 10,
            pageOptions: [5, 10, 15],
            sortBy: null,
        };
    },
    methods: {
        remove(item) {
            this.$store.dispatch('RemoveTable', item);
        },
        additem(){
            this.$store.dispatch('CreateTable',this.newitem);
            this.newitem = '';
        },
        Toggleall(){
            let all = [];
            if(!this.selectall){
                this.todos.forEach(function(data){
                    all.push(data.id);
                })

            }
            this.selectitems = all;
        }
    },
    created() {
        this.$store.dispatch('FetchSummaryTables');
    },
    computed: {
        todos() {
            this.selectitems = [];
            return this.$store.getters.tableSelectForm;
        },
        totalrows() {
            return this.$store.getters.totalTables;
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
