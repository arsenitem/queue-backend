<template>
<div>
    <div class="row">
        <div class="col s12 xl10 offset-xl1">
            <div class="row">
                <h3>Создание очереди</h3>
                <div class="row">
                    <form class="col s12">
                        <div class="row">
                            <div class="input-field col s12 xl4">
                                <input placeholder="Не указано" id="name_queque" type="text" class="validate">
                                <label for="name_queque">Наименование очереди</label>
                            </div>
                            <div class="input-field col s12 xl4">
                                <input type="text" placeholder="Не указано" id="date_start" class="datepicker">
                                <label for="date_start">Дата открытия</label>
                            </div>
                            <div class="input-field col s12 xl4">
                                <input type="text" placeholder="Не указано" id="date_end" class="datepicker">
                                <label for="date_end">Дата закрытия</label>
                            </div>                            
                        </div>
                        <div class="row custom-input">
                            <table id="parametersTable" class="responsive-table centered">
                                <thead>
                                    <tr>
                                        <th>Имя параметра</th>
                                        <th>Учавствует ли в сортировке?</th>
                                        <th>Коэффицент рейтинга</th>
                                        <th>По убывание, возрастанию</th>
                                        <th>Обязательный</th>
                                        <th>Скрыто</th>
                                        <th>Порядок отображения</th>
                                    </tr>
                                </thead>                       
                                <tbody>                         
                                </tbody>
                            </table>
                        </div>
                        <a class="waves-effect waves-light btn" onclick="createCustomModal('Добавить параметр', fillModalContentParameters)"><i class="material-icons left">add_circle_outline</i>Добавить параметр</a>
                        <a class="waves-effect waves-light btn" onclick="buildParameters()"><i class="material-icons left">play_circle_filled</i>Создать очередь</a>
                    </form>
                </div>               
            </div>
        </div>
    </div> 
</div>
</template>
<script>
    M.AutoInit();
</script>