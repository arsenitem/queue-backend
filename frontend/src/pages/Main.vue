<template>
<div>
    <main>
    <div class="row">
        <div class="col s12 xl10 offset-xl1">
            <div class="row">
                <h3>Очереди</h3>
                <a v-on:click="getAttr" class="waves-effect waves-light btn"><i class="material-icons left">sync</i>Обновить</a>
                <div class="col s12">
                    <table id="queueTable">
                        <thead>
                            <tr>
                                <th>Наименование</th>
                                <th>Ключ</th>
                            </tr>
                        </thead>               
                        <tbody v-on:click="goToAncets">
                            
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    </div>    
</main>
</div>
</template>
<script>
M.AutoInit();
    import * as Exonum from 'exonum-client'
    import axios from 'axios'
    import * as proto from '../../proto/stubs.js'

    const TRANSACTION_URL = '/api/explorer/v1/transactions';
    const PER_PAGE = 10;
    const SERVICE_ID = 10;
    const TX_TRANSFER_ID = 0;
    const TX_ISSUE_ID = 1;
    const TX_WALLET_ID = 0;
    const TABLE_INDEX = 0;
module.exports = {
    methods: {   
        getAttr(){
            axios.get('/api/services/queue_constructor/v1/queue_constructor/get_all_queues').then(function(value){
                $("#queueTable tbody").html("");
                value.data.forEach(function(value){
                    let key = Exonum.uint8ArrayToHexadecimal(new Uint8Array(value.key.data)),
                    name = value.name;
                    $("#queueTable tbody").append("<tr><td>"+name+"</td><td>"+key+"</td></tr>");
                });
                M.toast({html: 'Очереди возвращены'});
            });
        },
        goToAncets() {
            this.$router.push('profile');
        }
    }
}
</script>