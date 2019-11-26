<template>
<div>
    <div class="preloader">
        <div class="preloader-wrapper big active">
            <div class="spinner-layer spinner-blue-only">
                <div class="circle-clipper left">
                    <div class="circle"></div>
                </div><div class="gap-patch">
                    <div class="circle"></div>
                </div><div class="circle-clipper right">
                    <div class="circle"></div>
                </div>
            </div>
        </div>
    </div>
    <div class="custom-modals"></div>
    <ul id="login-sidenav" class="sidenav">
        <li>
            <div class="user-view">
                <div class="background">
                    <img src="img/background.jpg">
                </div>
                <a href="#user"><img class="circle" src="img/150.png"></a>
                <a href="#name"><span class="white-text name">Вход в аккаунт</span></a>
                <a href="#email"><span class="white-text email">Заполните поля</span></a>
            </div>
        </li>
        <li>
            <div class="row">
                <form class="col s12">
                    <div class="row">
                        <div class="input-field col s12">
                            <span>Логин</span>
                            <input id="name_login" type="text" class="validate">
                        </div>
                        <div class="input-field col s12">
                            <span>Пароль</span>
                            <input id="password_login" type="password" class="validate">
                        </div>
                    </div>
                    <button class="btn waves-effect waves-light" name="action">Войти
                        <i class="material-icons right">send</i>
                    </button>
                </form>
            </div>
        </li>
    </ul>
    <nav class="main">
        <div class="nav-wrapper">
            <div class="row">
                <div class="col s12">
                    <a href="index.html" class="brand-logo">Очередь на блокчейне</a>
                    <ul class="right hide-on-med-and-down">
                        <li><a href="#/main">Очереди</a></li>
                        <li><a href="/">Создать очередь</a></li>
                        <li><a href="#/profile">Анкеты</a></li>                      
                    </ul>
                </div>
            </div> 
            
        </div>
    </nav>
    <router-view/>

    <footer class="page-footer">
        <div class="container">
            <div class="row">
                <div class="col l6 s12">
                    <h5 class="white-text">Footer Content</h5>
                    <p class="grey-text text-lighten-4">You can use rows and columns here to organize your footer content.</p>
                </div>
                <div class="col l4 offset-l2 s12">
                    <h5 class="white-text">Links</h5>
                    <ul>
                    <li><a class="grey-text text-lighten-3" href="#!">Link 1</a></li>
                    <li><a class="grey-text text-lighten-3" href="#!">Link 2</a></li>
                    <li><a class="grey-text text-lighten-3" href="#!">Link 3</a></li>
                    <li><a class="grey-text text-lighten-3" href="#!">Link 4</a></li>
                    </ul>
                </div>
            </div>
        </div>
        <div class="footer-copyright">
            <div class="container">
                © 2019 Очередь на блокчейне
                <a class="grey-text text-lighten-4 right" href="#!">More Links</a>
            </div>
        </div>
    </footer>
  </div>
</template>

<style>
  input:invalid {
    box-shadow: none;
  }

  code {
    word-break: break-all;
    word-wrap: break-word;
  }
</style>
<script>
M.AutoInit();
    $('#mainNav li .main-link').click(function(event){
        event.preventDefault();
        linkLocation = this.href;
        $("body").fadeOut(600, redirectPage);
    });
    function redirectPage() {
        window.location = linkLocation;
    }
    let preloaderChecker = true;    
    window.onload = function() {
        if (preloaderChecker){
            $('.preloader').fadeOut();
            preloaderChecker = false;
        }      
    };
    window.setTimeout(function (){
        if (preloaderChecker){
            $('.preloader').fadeOut();
            preloaderChecker = false;
        }
    }, 1000);
export default {
  
}
</script>
