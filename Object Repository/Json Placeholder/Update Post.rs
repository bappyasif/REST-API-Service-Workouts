<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Update Post</description>
   <name>Update Post</name>
   <tag></tag>
   <elementGuidId>46db8d76-e28c-4a98-9013-b365ddcefce3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;id\&quot;: \&quot;2\&quot;,\n  \&quot;title\&quot;: \&quot;${post_title}\&quot;,\n  \&quot;body\&quot;: \&quot;some somenskcsjgbisnv nsisanvnjknvoge ckjsijnslkb Updated\&quot;,\n  \&quot;userId\&quot;: \&quot;${user_id}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/posts/1</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>26</defaultValue>
      <description></description>
      <id>6f1eef7f-5de8-4917-9669-d7885f17bc05</id>
      <masked>false</masked>
      <name>user_id</name>
   </variables>
   <variables>
      <defaultValue>'Updated Recently'</defaultValue>
      <description></description>
      <id>914e4533-76f8-4415-9c2c-21f9732016de</id>
      <masked>false</masked>
      <name>post_title</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//WS.verifyElementPropertyValue(response, 'title', '')
WS.verifyElementPropertyValue(response, 'title', 'Updated Recently')
WS.verifyElementPropertyValue(response, 'userId', '26')</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
