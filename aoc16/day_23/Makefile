DEPS = assembunny
JARS = $(DEPS:%=../%/lib.jar)
CP = $(if $(JARS),$(JARS:$() $()=:),"")

default: run

%/lib.jar: %/lib.kt
	$(MAKE) -C $(dir $@) build

main.jar: main.kt $(JARS)
	kotlinc main.kt -include-runtime -cp $(CP) -d main.jar

run: main.jar
	java -cp main.jar:$(CP) MainKt
