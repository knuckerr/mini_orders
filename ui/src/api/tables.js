import api from './api'

export default {
    newTable(newitem){
        return api().post('/new_table?name=' + newitem)
    },
    DeleteTable(id) {
        return api().post('/del_table', [id])
    },

    ClearTables(selected) {
        return api().post('/clear_orders',selected)
    },
    GetSummaryTables(){
        return api().get('/sumary_tables');
    },
    DeleteTables(selected){
        return api().post('/del_table',selected)
    },
    GetSummaryTable(name) {
        return api().get('/table?name=' + name)
    },
    GenerateRangeTables(from,to){
        return api().post('/range_table?from='+from+'&to='+to)
    }

}
